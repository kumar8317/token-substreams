use crate::abi::erc20::events::{Approval as ERC20ApprovalEvent, Transfer as ERC20TransferEvent};
use crate::pb::zdexer::eth::erc20::v1::{Approval, Contract, Transfer, Contracts};
use crate::abi::erc20::functions as erc20_functions;
use common::format_with_0x;
use substreams::scalar::BigInt;
use substreams::{Hex, log};
use substreams_ethereum::pb::eth::rpc::RpcResponse;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use substreams_ethereum::rpc::RpcBatch;

/**
 * Batch RPC calls for all requested addresses
 */
fn fetch_data(token_addresses: Vec<String>) -> Vec <RpcResponse>{
    let mut responses = Vec::new();
    let mut batch = RpcBatch::new();

    for (i,token_address) in token_addresses.iter().enumerate() {
        let token_address_bytes=hex::decode(token_address).unwrap();
        batch=batch.add(
            erc20_functions::Decimals {},
            token_address_bytes.clone(),
        )
        .add(
            erc20_functions::Name {},
            token_address_bytes.clone(),
        )
        .add(
            erc20_functions::Symbol {},
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

pub fn get_contracts(token_addresses: Vec<String>) -> Contracts {
    let mut contracts = Contracts { items: vec![] };
    let token_addresses_copy = token_addresses.clone();
    let array_data = fetch_data(token_addresses);

    for response_group in array_data.chunks(3) {
        let decimals: u64;
        match RpcBatch::decode::<_, erc20_functions::Decimals>(&response_group[0]) {
            Some(decoded_decimals) => {
                let min_u64 = BigInt::from(u64::MIN);
                let max_u64 = BigInt::from(u64::MAX);

                if &decoded_decimals >= &min_u64 && &decoded_decimals <= &max_u64 {
                    decimals = decoded_decimals.to_u64();
                } else {
                    log::debug!("Overflow occurred while converting decoded_decimals to u64.");
                    decimals = 0;
                }
            }
            None => decimals = 0,
        }

        let name: String;
        match RpcBatch::decode::<_, erc20_functions::Name>(&response_group[1]) {
            Some(decoded_name) => {
                name = decoded_name;
            }
            None => name = String::from(""),
        };

        let symbol: String;
        match RpcBatch::decode::<_, erc20_functions::Symbol>(&response_group[2]) {
            Some(decoded_symbol) => {
                symbol = decoded_symbol;
            }
            None => symbol = String::from(""),
        };

        let token_address_index = contracts.items.len();
        contracts.items.push(Contract {
            name,
            decimals,
            symbol,
            token_address: format_with_0x(token_addresses_copy[token_address_index].clone()),
        });
    }
    contracts
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
                        log_index: log.block_index as u64
                    }),
                ];
            }

            vec![]
        })
    })
}
