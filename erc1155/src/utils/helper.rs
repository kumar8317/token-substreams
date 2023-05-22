use crate::abi::erc1155::events::{TransferBatch as ERC1155TransferBatchEvent, TransferSingle as ERC1155TransferSingleEvent , ApprovalForAll as ERC1155ApprovalForAllEvent};
use crate::pb::zdexer::eth::erc1155::v1::{Transfer, Operator};
use substreams::scalar::BigInt;
use substreams::{log, Hex};
use substreams_ethereum::block_view::ReceiptView;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
use common::format_with_0x;
use num_bigint::{BigInt as nBigInt, Sign};


pub fn get_transfers(blk: &eth::Block) -> impl Iterator<Item = Transfer> + '_ {
    blk.receipts().flat_map(move |receipt| {
        let hash = &receipt.transaction.hash;

        receipt.receipt.logs.iter().flat_map(move |log| {
            let value_native = match &receipt.clone().transaction.value {
                Some(b) => nBigInt::from_bytes_be(Sign::Plus, &b.bytes.to_vec()).to_string(),
                None => {
                    String::new()
                }
            };
            if let Some(event) = ERC1155TransferSingleEvent::match_and_decode(log) {
                let (from_balance, to_balance) =get_balances(
                    event.value.clone(),
                    receipt.transaction.calls.clone()
                );
                return vec![new_erc1155_single_transfer(
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
                    value_native,
                    from_balance,
                    to_balance
                )];
            }

            if let Some(event) = ERC1155TransferBatchEvent::match_and_decode(log) {
                return new_erc1155_batch_transfer(
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
                    value_native,
                    receipt
                );
            }

            vec![]
        })
    })
}

fn new_erc1155_single_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC1155TransferSingleEvent,
    ordinal: u64,
    address: &[u8],
    block_number: u64,
    timestamp: u64,
    block_hash:&[u8],
    transaction_index: u32,
    transaction_type: i32,
    transaction_intiator: &[u8],
    value_native: String,
    from_balance: BigInt,
    to_balance: BigInt,
) -> Transfer {
    new_erc1155_transfer(
        hash,
        log_index,
        &event.from,
        &event.to,
        &event.id,
        &event.value,
        &event.operator,
        ordinal,
        address,
        block_number,
        timestamp,
        block_hash,
        transaction_index,
        transaction_type,
        transaction_intiator,
        value_native,
        from_balance,
        to_balance
    )
}

fn new_erc1155_batch_transfer(
    hash: &[u8],
    log_index: u32,
    event: ERC1155TransferBatchEvent,
    ordinal: u64,
    address: &[u8],
    block_number: u64,
    timestamp: u64,
    block_hash:&[u8],
    transaction_index: u32,
    transaction_type: i32,
    transaction_intiator: &[u8],
    value_native: String,
    receipt: ReceiptView
) -> Vec<Transfer> {
    if event.ids.len() != event.values.len() {
        log::info!("There is a different count for ids ({}) and values ({}) in transaction {} for log at block index {}, ERC1155 spec says lenght should match, ignoring the log completely for now",
            event.ids.len(),
            event.values.len(),
            Hex(&hash).to_string(),
            log_index,
        );

        return vec![];
    }

    event
        .ids
        .iter()
        .enumerate()
        .map(|(i, id)| {
            let value = event.values.get(i).unwrap();
            let (from_balance, to_balance) =get_balances(
                value.clone(),
                receipt.transaction.calls.clone()
            );
            new_erc1155_transfer(
                hash,
                log_index,
                &event.from,
                &event.to,
                id,
                value,
                &event.operator,
                ordinal,
                address,
                block_number,
                timestamp,
                block_hash,
                transaction_index,
                transaction_type,
                transaction_intiator,
                value_native.clone(),
                from_balance,
                to_balance
            )
        })
        .collect()
}

fn new_erc1155_transfer(
    hash: &[u8],
    log_index: u32,
    from: &[u8],
    to: &[u8],
    token_id: &BigInt,
    quantity: &BigInt,
    operator: &[u8],
    ordinal: u64,
    address: &[u8],
    block_number: u64,
    timestamp: u64,
    block_hash:&[u8],
    transaction_index: u32,
    transaction_type: i32,
    transaction_intiator: &[u8],
    value_native: String,
    from_balance: BigInt,
    to_balance: BigInt,
) -> Transfer {
    Transfer {
        from: format_with_0x(Hex(from).to_string()),
        to: format_with_0x(Hex(to).to_string()),
        quantity: quantity.to_string(),
        trx_hash: format_with_0x(Hex(hash).to_string()),
        log_index: log_index as u64,
        operator: format_with_0x(Hex(operator).to_string()),
        token_id: token_id.to_string(),
        log_ordinal: ordinal,
        token_address: format_with_0x(Hex(address).to_string()),
        block_number,
        timestamp,
        block_hash: format_with_0x(Hex(block_hash).to_string()),
        transaction_index,
        transaction_type,
        transaction_initiator:format_with_0x(Hex(transaction_intiator).to_string()),
        value: value_native,
        balance_from: from_balance.to_string(),
        balance_to: to_balance.to_string()
    }
}

pub fn get_approvals(blk: &eth::Block) -> impl Iterator<Item = Operator> + '_ {
    blk.receipts().flat_map(|receipt| {

        receipt.receipt.logs.iter().flat_map(|log| {

            if let Some(event) = ERC1155ApprovalForAllEvent::match_and_decode(log) {
                return vec![(
                    Operator{
                        trx_hash: format_with_0x(Hex::encode(&receipt.transaction.hash)),
                        token_address: format_with_0x(Hex::encode(&log.address)),
                        token_id: String::from(""),
                        owner_address: format_with_0x(Hex::encode(event.account)),
                        operator_address: format_with_0x(Hex::encode(event.operator)),
                        approved: event.approved
                    }
                )];
            }

            vec![]
        })
    })
} 

fn get_balances(value: BigInt, calls: Vec<eth::Call>) -> (BigInt, BigInt) {
    let mut from_balance = BigInt::from(0);
    let mut to_balance = BigInt::from(0);
    for call in calls {
        for storage_change in call.storage_changes {

            let old_value = BigInt::from_unsigned_bytes_be(&storage_change.old_value);
            let new_value = BigInt::from_unsigned_bytes_be(&storage_change.new_value);
            let mut from = false;
            let mut amount =  new_value.clone()-old_value.clone();

            if amount < BigInt::from(0) {
                amount = amount.neg();
                from = true;
            }
            
            if amount == value {
                if from {
                    from_balance = new_value.clone();
                } else {
                    to_balance = new_value.clone();
                }
            }
        }
    }
    return (from_balance, to_balance);
}
