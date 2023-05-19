use crate::abi::erc721::events::{Transfer as ERC721TransferEvent, Approval as ERC721ApprovalEvent , ApprovalForAll as ERC721ApprovalForAllEvent};
use crate::pb::zdexer::eth::erc721::v1::{Collection, Transfer, Approval, Collections, CollectionOwner};
use crate::{abi::erc721::functions as erc721_functions};
use substreams::store::{StoreGetProto, StoreGet};
use substreams::{hex, Hex};
use substreams_ethereum::pb::eth::rpc::RpcResponse;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::rpc::RpcBatch;
use substreams_ethereum::Event;
use common::format_with_0x;
use num_bigint::{BigInt as nBigInt, Sign};
//pub const ERC721_IFACE_ID: [u8; 4] = hex!("01ffc9a7");
pub const ERC721_METADATA_IFACE_ID: [u8; 4] = hex!("5b5e139f");

// pub fn get_collections(
//     token_address: &String,
//     tx_hash: &str,
//     from: &str,
// ) -> Option<Collection> {
//     let token_address_bytes = Hex::decode(token_address).unwrap();

//     let eip721_iface_resp = erc721_functions::SupportsInterface {
//         interface_id: ERC721_IFACE_ID,
//     }
//     .call(token_address_bytes.clone());

//     if eip721_iface_resp == Some(true) {
//         let batch = RpcBatch::new();
//         let responses = batch
//             .add(erc721_functions::Name {}, token_address_bytes.clone())
//             .add(erc721_functions::Symbol {}, token_address_bytes.clone())
//             .add(
//                 erc721_functions::SupportsInterface {
//                     interface_id: ERC721_METADATA_IFACE_ID,
//                 },
//                 token_address_bytes,
//             )
//             .execute()
//             .unwrap()
//             .responses;

//         let name = match RpcBatch::decode::<_, erc721_functions::Name>(&responses[0]) {
//             Some(decoded_name) => decoded_name,
//             None => String::from(""),
//         };
//         log::debug!("decoded_name ok");

//         let symbol = match RpcBatch::decode::<_, erc721_functions::Symbol>(&responses[1]) {
//             Some(decoded_symbol) => decoded_symbol,
//             None => String::from(""),
//         };
//         log::debug!("decoded_symbol ok");

//         let supports_metadata =RpcBatch::decode::<_, erc721_functions::SupportsInterface>(&responses[2]).unwrap_or(false);
//         log::debug!("decoded supports_metadata ok");

//         return Some(Collection {
//             token_address: format_with_0x(token_address.clone()),
//             name,
//             symbol,
//             supports_metadata,
//             owner_address: format_with_0x(from.to_string()),
//             deploy_trx: format_with_0x(tx_hash.to_string()),
//         });
//     }
//     None
// }

/**
 * Batch RPC calls for all requested addresses
 */
fn fetch_data(token_addresses: Vec<String>) -> Vec <RpcResponse>{
    let mut responses = Vec::new();
    let mut batch = RpcBatch::new();

    for (i,token_address) in token_addresses.iter().enumerate() {
        let token_address_bytes=hex::decode(token_address).unwrap();
        batch=batch.add(
            erc721_functions::SupportsInterface {
                interface_id: ERC721_METADATA_IFACE_ID
            },
            token_address_bytes.clone(),
        )
        .add(
            erc721_functions::Name {},
            token_address_bytes.clone(),
        )
        .add(
            erc721_functions::Symbol {},
            token_address_bytes,
        );

        if (i + 1) % 50 == 0 || i == token_addresses.len() - 1 {
            let batch_response = batch
            .execute()
            .unwrap();
            responses.extend(batch_response.responses);
            batch = RpcBatch::new();
        }
       
    }
  return responses  
}

pub fn get_collections(token_addresses: Vec<String> , collection_owner_store: StoreGetProto<CollectionOwner>) -> Collections {
    let mut collections = Collections {items :vec![]};
    let token_addresses_copy = token_addresses.clone();
    let array_data: Vec<RpcResponse> = fetch_data(token_addresses);

    for response_group in array_data.chunks(3) { 
        let supports_metadata =RpcBatch::decode::<_, erc721_functions::SupportsInterface>(&response_group[0]).unwrap_or(false);
        let name = RpcBatch::decode::<_,erc721_functions::Name>(&response_group[1]).unwrap_or(String::from(""));
        let symbol = RpcBatch::decode::<_,erc721_functions::Symbol>(&response_group[2]).unwrap_or(String::from(""));
        let token_address_index = collections.items.len();
        let mut deploy_trx = String::new();
        let mut owner_address = String::new();
        let token_address = format_with_0x(token_addresses_copy[token_address_index].clone());
        
        match collection_owner_store.get_last(format_with_0x(token_address.clone())) {
            Some(data)=>{
                owner_address=data.owner_address;
                deploy_trx=data.deploy_trx;
            },
            None =>{

            }
        }

        collections.items.push({
            Collection {
                name,
                symbol,
                supports_metadata,
                owner_address,
                deploy_trx,
                token_address
            }
        })
        

    }
    collections
}   

pub fn get_transfers(blk: &eth::Block) -> impl Iterator<Item = Transfer> + '_ {
    blk.receipts().flat_map(move |receipt| {
        let hash = &receipt.transaction.hash;

        receipt.receipt.logs.iter().flat_map(move |log| {
            if let Some(event) = ERC721TransferEvent::match_and_decode(log) {

                let value = match &receipt.clone().transaction.value {
                    Some(b) => nBigInt::from_bytes_be(Sign::Plus, &b.bytes.to_vec()).to_string(),
                    None => {
                        String::new()
                    }
                };
                return vec![new_transfer(
                    hash,
                    log.block_index,
                    event,
                    log.ordinal,
                    &log.address,
                    blk.number,
                    blk.timestamp_seconds(),
                    &blk.hash,
                    receipt.transaction.index,
                    receipt.transaction.r#type,
                    &receipt.transaction.from,
                    value,
                )];
            }
            vec![]
        })
    })
}

fn new_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC721TransferEvent,
    log_ordinal: u64,
    token_address: &[u8],
    block_number: u64,
    timestamp: u64,
    block_hash:&[u8],
    transaction_index: u32,
    transaction_type: i32,
    transaction_intiator: &[u8],
    value: String,
) -> Transfer {
    Transfer {
        token_address: format_with_0x(Hex::encode(&token_address)),
        from: format_with_0x(Hex::encode(&event.from)),
        to: format_with_0x(Hex::encode(&event.to)),
        token_id: event.token_id.to_string(),
        trx_hash: format_with_0x(Hex::encode(hash)),
        log_index: log_index as u64,
        log_ordinal,
        block_number,
        timestamp,
        block_hash: format_with_0x(Hex(block_hash).to_string()),
        transaction_index,
        transaction_type,
        transaction_initiator:format_with_0x(Hex(transaction_intiator).to_string()),
        value
    }
}

pub fn get_approvals(blk: &eth::Block) -> impl Iterator<Item = Approval> + '_ {
    blk.receipts().flat_map(|receipt| {

        receipt.receipt.logs.iter().flat_map(|log| {
            if let Some(event) = ERC721ApprovalEvent::match_and_decode(log) {
                return vec![(
                   Approval{
                        trx_hash: format_with_0x(Hex::encode(&receipt.transaction.hash)),
                        token_address: format_with_0x(Hex::encode(&log.address)),
                        token_id: event.token_id.to_string(),
                        owner_address: format_with_0x(Hex::encode(event.owner)),
                        operator_address: format_with_0x(Hex::encode(event.approved)),
                        approved: true,
                        log_index: log.block_index as u64
                    }
                )];
            }

            if let Some(event) = ERC721ApprovalForAllEvent::match_and_decode(log) {
                return vec![(
                    Approval{
                        trx_hash: format_with_0x(Hex::encode(&receipt.transaction.hash)),
                        token_address: format_with_0x(Hex::encode(&log.address)),
                        token_id: String::from(""),
                        owner_address: format_with_0x(Hex::encode(event.owner)),
                        operator_address: format_with_0x(Hex::encode(event.operator)),
                        approved: event.approved,
                        log_index: log.block_index as u64
                    }
                )];
            }

            vec![]
        })
    })
} 
