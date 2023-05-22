mod abi;
#[path = "./db_out.rs"]
mod db;
#[path = "./graph_out.rs"]
mod graph;
#[allow(dead_code)]
mod pb;
mod rpc;
mod utils;

use std::collections::HashMap;
use std::str::FromStr;

use common:: remove_0x;
use pb::zdexer::eth::erc1155::v1::{ Mint, Mints, Operators, Token, Tokens, Transfers, Address};
use rpc::RpcTokenURI;
use substreams::store::{
     DeltaProto, Deltas, StoreAdd, StoreAddBigInt, StoreNew, StoreSet, StoreSetProto, StoreSetIfNotExistsProto,
    StoreSetIfNotExists
};
use substreams::{errors::Error, log, scalar::BigInt, Hex};
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_ethereum::NULL_ADDRESS;
use substreams_ethereum::{pb::eth::v2 as eth, rpc::RpcBatch};
use utils::helper::get_approvals;
use utils::helper::get_transfers;
use utils::keyer;

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
fn map_approvals(blk: eth::Block) -> Result<Operators, Error> {
    Ok(Operators {
        items: get_approvals(&blk).collect(),
    })
}

#[substreams::handlers::map]
fn map_extract_mints(transfers: Transfers) -> Result<Mints, Error> {
    let mut map: HashMap<(String, String), Mint> = HashMap::new();

    for transfer in transfers.items {
        let token_address = transfer.token_address.clone();
        let token_id = transfer.token_id.clone();

        if map
            .get_mut(&(token_address.clone(), token_id.clone()))
            .is_some()
        {
        } else {
            
            if Hex::decode(remove_0x(&transfer.from.clone())).unwrap() == NULL_ADDRESS {
               let minter_address= transfer.transaction_initiator;
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
    let mut tokens = Tokens { items: vec![] };
    if !transfers.items.is_empty() {
        let mut array_rpc_calls: Vec<RpcTokenURI> = vec![];
        let clonearray = transfers.items.clone();
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

        for (index, transfer) in transfers.items.into_iter().enumerate() {
            let metadata_uri =
                match RpcBatch::decode::<_, abi::erc1155::functions::Uri>(&rpc_responses[index]) {
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
fn store_tokens(tokens: Tokens, output: StoreSetProto<Token>) {
    for token in tokens.items {
        output.set(
            1,
            keyer::token_store_key(&token.token_address, &token.token_id),
            &token,
        )
    }
}

#[substreams::handlers::store]
fn store_balance(transfers: Transfers, output: StoreAddBigInt) {
    log::info!("Stored events {}", transfers.items.len());
    for transfer in transfers.items {
        output.add(
            transfer.log_ordinal,
            keyer::balance_key(
                &transfer.to,
                &transfer.token_address,
                &transfer.token_id.clone(),
            ),
            &BigInt::from_str((transfer.quantity).as_str()).unwrap(),
        );

        if Hex::decode(remove_0x(&transfer.from.clone())).unwrap() != NULL_ADDRESS {
            output.add(
                transfer.log_ordinal,
                keyer::balance_key(&transfer.from, &transfer.token_address, &transfer.token_id),
                &BigInt::from_str((transfer.quantity).as_str())
                    .unwrap()
                    .neg(),
            );
        }
    }
}

#[substreams::handlers::map]
fn map_transfer_entities(transfers: Transfers) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    graph::transfer_entity_change(&mut entity_changes, transfers);

    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_token_entities(tokens: Tokens, mints: Mints) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::token_create_entity_changes(&mut entity_changes, tokens);
    graph::mints_token_entity_changes(&mut entity_changes, mints);
    Ok(entity_changes)
}

#[substreams::handlers::map]
fn map_operator_entities(approvals: Operators) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    graph::approval_operator_entity_changes(&mut entity_changes, approvals);
    Ok(entity_changes)
}


#[substreams::handlers::map]
fn graph_out(
    token_entities: EntityChanges,
    transfer_entities: EntityChanges,
    operator_entities: EntityChanges,
) -> Result<EntityChanges, Error> {
    Ok(EntityChanges {
        entity_changes: [
            token_entities.entity_changes,
            transfer_entities.entity_changes,
            operator_entities.entity_changes,
        ]
        .concat(),
    })
}


#[substreams::handlers::map]
fn map_transfers_db(transfers: Transfers) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::transfer_db_changes(&mut database_changes, transfers);

    Ok(database_changes)
}

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
fn map_operators_db(approvals: Operators) -> Result<DatabaseChanges, Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    db::approval_operator_db_changes(&mut database_changes, approvals);

    Ok(database_changes)
}

#[substreams::handlers::map]
fn db_out(
    tokens_db: DatabaseChanges,
    transfers_db: DatabaseChanges,
    approvals_db: DatabaseChanges,
) -> Result<DatabaseChanges, Error> {
    Ok(DatabaseChanges {
        table_changes: [
            tokens_db.table_changes,
            transfers_db.table_changes,
            approvals_db.table_changes,
        ]
        .concat(),
    })
}
