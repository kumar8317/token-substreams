mod abi;
#[path = "./graph_out.rs"]
mod graph;
#[allow(dead_code)]
mod pb;
mod rpc;
mod utils;
#[path="./db_out.rs"]
mod db;

use std::collections::HashMap;
use std::str::FromStr;

use pb::zdexer::eth::erc721::v1::{
    Approvals, Collections, Mint, Mints, Token, Tokens,
    Transfers, Transfer, CollectionOwner, Address
};
use rpc::RpcTokenURI;
use substreams::store::{StoreSetProto, StoreNew, StoreSet, Deltas, DeltaProto, StoreGetProto, StoreSetIfNotExistsProto, StoreGet, StoreSetIfNotExists};
use substreams::{errors::Error, hex, scalar::BigInt, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::NULL_ADDRESS;
use substreams_ethereum::{pb::eth::v2 as eth, rpc::RpcBatch};
use utils::helper::get_approvals;
use common::{remove_0x, format_with_0x};
use utils::helper::{self, get_transfers};
use common::pb::zdexer::eth::events::v1::OwnershipTransfers;
use utils::keyer;

const INITIALIZE_METHOD_HASH: [u8; 4] = hex!("1459457a");

#[substreams::handlers::store]
fn store_collections_owners(blk: eth::Block, output: StoreSetIfNotExistsProto<CollectionOwner>) {

    for call_view in blk.calls() {
        let tx_hash = Hex(&call_view.transaction.hash).to_string();
        let from = Hex(&call_view.transaction.from).to_string();

        let call = call_view.call;
        if call.call_type == eth::CallType::Create as i32 {
            let call_input_len = call.input.len();
            if call.call_type == eth::CallType::Call as i32
                && (call_input_len < 4 || call.input[0..4] != INITIALIZE_METHOD_HASH)
            {
                // this will check if a proxy contract has been called to create a contract.
                // if that is the case the Proxy contract will call the initialize function on the contract
                // this is part of the OpenZeppelin Proxy contract standard
                //log::debug!("{:?}if false--- ", INITIALIZE_METHOD_HASH);
                continue;
            }

            let address = Hex(&call.address).to_string();

            output.set_if_not_exists(
                1,
                &format_with_0x(address.clone()),
                &CollectionOwner {
                    token_address: format_with_0x(address),
                    owner_address: format_with_0x(from),
                    deploy_trx: format_with_0x(tx_hash),
                }
            );
        }
    }


}

// #[substreams::handlers::map]
// fn map_collections(blk: eth::Block) -> Result<Collections, Error> {
//     let mut erc721_collections = Collections { items: vec![] };

//     for call_view in blk.calls() {
//         let tx_hash = Hex(&call_view.transaction.hash).to_string();
//         let from = Hex(&call_view.transaction.from).to_string();

//         let call = call_view.call;
//         if call.call_type == eth::CallType::Create as i32 {
//             let call_input_len = call.input.len();
//             if call.call_type == eth::CallType::Call as i32
//                 && (call_input_len < 4 || call.input[0..4] != INITIALIZE_METHOD_HASH)
//             {
//                 // this will check if a proxy contract has been called to create a contract.
//                 // if that is the case the Proxy contract will call the initialize function on the contract
//                 // this is part of the OpenZeppelin Proxy contract standard
//                 //log::debug!("{:?}if false--- ", INITIALIZE_METHOD_HASH);
//                 continue;
//             }

//             let address = Hex(&call.address).to_string();

//             log::info!("address {}", address);

//             let collection = helper::get_collections(&address, &tx_hash, &from);
//             if collection.is_some() {
//                 erc721_collections.items.push(collection.unwrap());
//             }
//         }
//     }

//     Ok(erc721_collections)
// }

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<Transfers, Error> {
    Ok(Transfers {
        items: get_transfers(&blk).collect(),
    })
}

#[substreams::handlers::store]
fn store_address(transfers: Transfers, output: StoreSetIfNotExistsProto<Address>) {
    for transfer in transfers.items {
        output.set_if_not_exists(
            transfer.log_ordinal,
            &transfer.token_address.clone(),
            &Address{ address: transfer.token_address.clone() },
        );
    }
}

#[substreams::handlers::map]
fn map_collections(deltas: Deltas<DeltaProto<Address>>, collection_owner_store: StoreGetProto<CollectionOwner>) -> Result<Collections, Error> {
    
    let mut array_addresses = vec![];
    for delta in deltas.deltas {
        let token_address = delta.new_value.address;
        array_addresses.push(remove_0x(&token_address));
    }
    let collections = helper::get_collections(array_addresses,collection_owner_store);
    Ok(collections)
}

#[substreams::handlers::map]
fn map_approvals(blk: eth::Block) -> Result<Approvals, Error> {
    Ok(Approvals {
        items: get_approvals(&blk).collect(),
    })
}

#[substreams::handlers::map]
fn map_extract_mints(transfers: Transfers) -> Result<Mints, Error> {
    let mut map: HashMap<(String, String), Mint> = HashMap::new();

    for transfer in transfers.items {
        let token_address = transfer.token_address.clone();
        let token_id = transfer.token_id.clone();

        if map.get_mut(&(token_address.clone(), token_id.clone())).is_some() {
        } else {
            
                if Hex::decode(remove_0x(&transfer.from.clone())).unwrap() == NULL_ADDRESS {
                   let  minter_address= transfer.transaction_initiator;
                    let mint = Mint {
                        token_address: transfer.token_address,
                        token_id: transfer.token_id,
                        minter_address,
                        mint_block: transfer.block_number,
                        min_trx: transfer.trx_hash,
                    };
        
                    map.insert((token_address, token_id), mint);
                };

            
        }
    }

    Ok(Mints {
        items: map.into_iter().map(|(_, mint)| mint).collect(),
    })
}

#[substreams::handlers::map]
fn map_extract_tokens(transfers: Transfers) -> Result<Tokens, Error> {

    //extract unique transfers
    let mut unique_transfers_map: HashMap<(String,String),Transfer> = HashMap::new();

    
    for transfer in transfers.items {
        let token_address = transfer.token_address.clone();
        let token_id = transfer.token_id.clone();

        if unique_transfers_map.get_mut(&(token_address.clone(), token_id.clone())).is_some() {

        }else{
            unique_transfers_map.insert((token_address,token_id), transfer);
        }
        
    }

    let unique_transfers: Vec<Transfer> = unique_transfers_map.into_iter().map(|(_,transfer)| transfer).collect();

    let mut tokens = Tokens { items: vec![] };
    if !unique_transfers.is_empty() {
        let mut array_rpc_calls: Vec<RpcTokenURI> = vec![];
        let clonearray = unique_transfers.clone();
        for transfer in clonearray {
            let token_id_bigint = BigInt::from_str(&transfer.token_id).unwrap();

            let param = RpcTokenURI {
                to: Hex::decode(remove_0x(&transfer.token_address.clone())).unwrap(),
                tokenid: token_id_bigint,
            };

            array_rpc_calls.push(param);
        }

        let rpc_responses = rpc::fetch_token_uri(array_rpc_calls);

        // let mut index = 0;

        for (index,transfer) in unique_transfers.into_iter().enumerate() {
            let metadata_uri = match RpcBatch::decode::<_, abi::erc721::functions::TokenUri>(
                &rpc_responses[index],
            ) {
                Some(data) => data,
                None => String::from(""),
            };

            let token = Token {
                token_address: transfer.token_address,
                token_id: transfer.token_id,
                metadata_uri,
                owner_address: transfer.to,
                block_number: transfer.block_number,
            };
            tokens.items.push(token);
        }
    }
    Ok(tokens)
}

#[substreams::handlers::store]
fn store_tokens(tokens: Tokens, output: StoreSetProto<Token>)
{
    for token in tokens.items {
        output.set(
            1,
            keyer::token_store_key(&token.token_address, &token.token_id),
            &token
        )
    }
}

#[substreams::handlers::map]
fn map_collection_entities(collections: Collections, ownership_transfers: OwnershipTransfers)-> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    graph::collection_entity_change(&mut entity_changes, collections);
    graph::collection_ownership_update_entity_change(&mut entity_changes, ownership_transfers);
    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_transfer_entities(transfers: Transfers) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    graph::transfer_entity_change(&mut entity_changes, transfers);

    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_token_entities(
    tokens: Tokens,
    mints: Mints,
) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::token_create_entity_changes(&mut entity_changes, tokens);
    graph::mints_token_entity_changes(&mut entity_changes, mints);
    Ok(entity_changes)
}


#[substreams::handlers::map]
fn map_operator_entities(approvals: Approvals) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::approval_operator_entity_changes(&mut entity_changes, approvals);
    Ok(entity_changes)
}

#[substreams::handlers::map]
fn graph_out(
    collection_entities: EntityChanges,
    token_entities: EntityChanges,
    transfer_entities: EntityChanges,
    operator_entities: EntityChanges,
) -> Result<EntityChanges, Error> {
    Ok(EntityChanges {
        entity_changes: [
            collection_entities.entity_changes,
            token_entities.entity_changes,
            transfer_entities.entity_changes,
            operator_entities.entity_changes,
        ]
        .concat(),
    })
}

#[substreams::handlers::map]
fn map_collections_db(collections: Collections,ownership_transfers: OwnershipTransfers) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::collection_db_changes(&mut database_changes, collections);
    db::collection_ownership_update_db_changes(&mut database_changes, ownership_transfers);

    Ok(database_changes)
}

#[substreams::handlers::map]
fn map_transfers_db(transfers: Transfers) -> Result<DatabaseChanges, Error> {
    let mut database_changes:DatabaseChanges = Default::default();

    db::transfer_db_changes(&mut database_changes, transfers);

    Ok(database_changes)
}

// #[substreams::handlers::map]
// fn map_tokens_db(
//     tokens: Tokens,
//     mints: Mints,
// ) -> Result<DatabaseChanges, Error> {
//     let mut database_changes: DatabaseChanges = Default::default();
//     db::token_db_changes(&mut database_changes, tokens);
//     db::mints_token_db_changes(&mut database_changes, mints);
//     Ok(database_changes)
// }

#[substreams::handlers::map]
fn map_tokens_db(
    token_deltas: Deltas<DeltaProto<Token>>,
    mints: Mints,
) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();
    db::token_db_changes(&mut database_changes, token_deltas);
    db::mints_token_db_changes(&mut database_changes, mints);
    Ok(database_changes)
}

#[substreams::handlers::map]
fn map_operators_db(approvals: Approvals) -> Result<DatabaseChanges, Error> {
    let mut database_changes:DatabaseChanges = Default::default();

    db::approval_operator_db_changes(&mut database_changes, approvals);

    Ok(database_changes)
}

#[substreams::handlers::map]
fn db_out(
    collections_db: DatabaseChanges,
    tokens_db: DatabaseChanges,
    transfers_db: DatabaseChanges,
    approvals_db: DatabaseChanges,
) -> Result<DatabaseChanges, Error> {
    Ok(DatabaseChanges {
        table_changes: [
            collections_db.table_changes,
            tokens_db.table_changes,
            transfers_db.table_changes,
            approvals_db.table_changes,
        ]
        .concat()
    })
}