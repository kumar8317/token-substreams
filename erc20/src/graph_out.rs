use std::str::FromStr;

use substreams::scalar::BigInt;
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use crate::{
    pb::zdexer::eth::erc20::v1::{Contracts, Approvals, Transfers},
    utils::{
        keyer::{operator_key, transfer_key},
    },
};

pub fn contract_entity_changes(changes: &mut EntityChanges, contracts: Contracts) {
    for contract in contracts.items {
        let key = contract.token_address.clone();
        changes
            .push_change("ERC20Contract", &key, 1, Operation::Create)
            .change("id", contract.token_address)
            .change("name", contract.name)
            .change("symbol", contract.symbol)
            .change("decimals", contract.decimals);
    }
}

pub fn account_create_entity_changes(changes: &mut EntityChanges, account_address: String) {
    changes
        .push_change("Account", &account_address, 1, Operation::Create)
        .change("id", account_address);
}

pub fn transfer_entity_changes(changes: &mut EntityChanges, transfers: Transfers) {
    for transfer in transfers.items {
        account_create_entity_changes(changes, transfer.from.clone());
        account_create_entity_changes(changes, transfer.to.clone());
        let key = transfer_key(transfer.block_number, transfer.log_index);
        changes
            .push_change(
                "ERC20Transfer",
                &key,
                transfer.log_ordinal,
                Operation::Create,
            )
            .change("id", key)
            .change("contract", transfer.token_address.clone())
            .change("trx_hash", transfer.trx_hash)
            .change("from_address", transfer.from)
            .change("to_address", transfer.to)
            .change("quantity", BigInt::from_str(&transfer.quantity).unwrap())
            .change("log_index", transfer.log_index)
            .change("block_number", transfer.block_number)
            .change("block_hash", transfer.block_hash)
            .change("timestamp", transfer.timestamp)
            .change("transaction_index", transfer.transaction_index as u64)
            .change("transaction_type", transfer.transaction_type)
            .change("from_balance", BigInt::from_str(&transfer.balance_from).unwrap())
            .change("to_balance", BigInt::from_str(&transfer.balance_to).unwrap());
    }
}


pub fn approval_entity_changes(changes: &mut EntityChanges, approvals: Approvals) {
    for approval in approvals.items {
        account_create_entity_changes(changes, approval.owner_address.clone());
        account_create_entity_changes(changes, approval.spender_address.clone());
        let key = operator_key(&approval.trx_hash, approval.log_index);
        changes
            .push_change("ERC20Approval", &key, 1, Operation::Create)
            .change("id", key)
            .change("contract", approval.token_address)
            .change("owner", approval.owner_address)
            .change("spender", approval.spender_address)
            .change("quantity", BigInt::from_str(&approval.quantity).unwrap());
    }
}
