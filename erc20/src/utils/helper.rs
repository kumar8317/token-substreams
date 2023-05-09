use crate::abi::erc20::events::{Approval as ERC20ApprovalEvent, Transfer as ERC20TransferEvent};
use crate::pb::zdexer::eth::erc20::v1::{Approval, Contract, Transfer};
use crate::abi::erc20::functions as erc20_functions;
use common::format_with_0x;
use substreams::{log, Hex};
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth as ethpb;
use substreams_ethereum::rpc::RpcBatch;
use substreams_ethereum::pb::eth::rpc::{RpcCalls,RpcCall};
use substreams_ethereum::Event;

pub const DECIMALS: &str = "313ce567";

pub fn create_rpc_calls(addr: &Vec<u8>, method_signatures: Vec<&str>) -> RpcCalls {
    let mut rpc_calls = RpcCalls { calls: vec![] };

    for method_signature in method_signatures {
        rpc_calls.calls.push(RpcCall {
            to_addr: Vec::from(addr.clone()),
            data: hex::decode(method_signature).unwrap(),
        })
    }

    return  rpc_calls
}

pub fn read_uint32(input: &[u8]) -> Result<u32, String> {
    if input.len() != 32 {
        return Err(format!("uint32 invalid length: {}", input.len()));
    }
    let as_array: [u8; 4] = input[28..32].try_into().unwrap();
    Ok(u32::from_be_bytes(as_array))
}

pub fn get_contracts(token_address: &String, tx_hash: &str, from: &str) -> Option<Contract> {
    let token_address_bytes = Hex::decode(token_address).unwrap();
    
    let rpc_call_decimal = create_rpc_calls(&token_address_bytes, vec![DECIMALS]);

    let rpc_response_unmarshalled_decimal: ethpb::rpc::RpcResponses = substreams_ethereum::rpc::eth_call(&rpc_call_decimal);

    let response_decimal = rpc_response_unmarshalled_decimal.responses;
    if !response_decimal[0].failed {

        let decoded_decimals = read_uint32(response_decimal[0].raw.as_ref());
        if decoded_decimals.is_err() {
            return None;
        }else{
            let decimals = decoded_decimals.unwrap() as u64;

            log::debug!("decimals:{}",decimals);
            let batch = RpcBatch::new();
            let responses = batch
                .add(erc20_functions::Name {}, token_address_bytes.clone())
                .add(erc20_functions::Symbol {}, token_address_bytes.clone())
                .execute()
                .unwrap()
                .responses;
        
            let name = match RpcBatch::decode::<_, erc20_functions::Name>(&responses[0]) {
                Some(decoded_name) => decoded_name,
                None => return None,
            };
            log::debug!("decoded_name ok");
        
            let symbol = match RpcBatch::decode::<_, erc20_functions::Symbol>(&responses[1]) {
                Some(decoded_symbol) => decoded_symbol,
                None => return None,
            };
            log::debug!("decoded_symbol ok");
        
            return Some(Contract {
                token_address: format_with_0x(token_address.clone()),
                name,
                symbol,
                decimals,
                owner_address: format_with_0x(from.to_string()),
                deploy_trx: format_with_0x(tx_hash.to_string()),
            })
        }
        
    }

    None
}

pub fn get_transfers(blk: &eth::Block) -> impl Iterator<Item = Transfer> + '_ {
    blk.receipts().flat_map(|receipt| {
        let hash = &receipt.transaction.hash;
        
        receipt.receipt.logs.iter().flat_map(|log| {
            if let Some(event) = ERC20TransferEvent::match_and_decode(log) {
                return vec![new_erc20_transfer(
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
                )];
            }
             vec![]
        })
    })
}

fn new_erc20_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC20TransferEvent,
    ordinal: u64,
    address: &[u8],
    block_number: u64,
    timestamp: u64,
    block_hash:&[u8],
    transaction_index: u32,
    transaction_type: i32
) -> Transfer {
    Transfer {
        from: format_with_0x(Hex(&event.from).to_string()),
        to: format_with_0x(Hex(&event.to).to_string()),
        quantity: event.value.to_string(),
        trx_hash: format_with_0x(Hex(hash).to_string()),
        log_index: log_index as u64,
        log_ordinal: ordinal,
        token_address: format_with_0x(Hex(address).to_string()),
        block_number,
        timestamp,
        block_hash: format_with_0x(Hex(block_hash).to_string()),
        transaction_index,
        transaction_type
    }
}

pub fn get_approvals(blk: &eth::Block) -> impl Iterator<Item = Approval> + '_ {
    blk.receipts().flat_map(|receipt| {
        receipt.receipt.logs.iter().flat_map(|log| {
            if let Some(event) = ERC20ApprovalEvent::match_and_decode(log) {
                return vec![
                    (Approval {
                        trx_hash: format_with_0x(Hex::encode(&receipt.transaction.hash)),
                        token_address: format_with_0x(Hex::encode(&log.address)),
                        owner_address: format_with_0x(Hex::encode(event.owner)),
                        spender_address: format_with_0x(Hex::encode(event.spender)),
                        quantity: event.value.to_string(),
                    }),
                ];
            }

            vec![]
        })
    })
}
