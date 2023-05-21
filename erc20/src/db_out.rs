use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use common::sanitize_string;
use crate::{pb::zdexer::eth::erc20::v1::{Transfers, Approvals, Contracts}, utils::keyer::{transfer_key, operator_key}};

pub fn contract_db_changes(
    changes:&mut DatabaseChanges,
    contracts: Contracts
) {
    for contract in contracts.items {
        //account_db_changes(changes, contract.owner_address.clone());
        let key = contract.token_address.clone();
        changes
            .push_change("erc20_contract", &key, 1, Operation::Create)
            .change("name", (None,sanitize_string(&contract.name)))
            .change("symbol", (None,sanitize_string(&contract.symbol)))
            .change("decimals", (None,contract.decimals));
    }
}


pub fn transfer_db_changes(
    changes:&mut DatabaseChanges,
    transfers: Transfers
) {
    for transfer in transfers.items {
       // account_db_changes(changes, transfer.from.clone());
       // account_db_changes(changes, transfer.to.clone());

        let key = transfer_key(transfer.block_number,transfer.log_index);
        changes
            .push_change("erc20_transfer", &key, transfer.log_ordinal, Operation::Create)
            .change("contract", (None,transfer.token_address.clone()))
            .change("trx_hash", (None,transfer.trx_hash))
            .change("from_address", (None,transfer.from))
            .change("to_address", (None,transfer.to))
            .change("quantity", (None,transfer.quantity))
            .change("log_index", (None,transfer.log_index))
            .change("block_number", (None,transfer.block_number))
            .change("block_hash", (None,transfer.block_hash))
            .change("timestamp", (None,transfer.timestamp))
            .change("transaction_index", (None,transfer.transaction_index as u64))
            .change("transaction_type", (None,transfer.transaction_type))
            .change("from_balance", (None,transfer.balance_from))
            .change("to_balance", (None,transfer.balance_to));
    }
}

pub fn approval_db_changes(
    changes:&mut DatabaseChanges,
    approvals: Approvals
) {
    for approval in approvals.items {
       // account_db_changes(changes, approval.owner_address.clone());
       // account_db_changes(changes, approval.spender_address.clone());

        let key = operator_key(&approval.trx_hash, approval.log_index);

        changes
            .push_change("erc20_approval", &key, 1, Operation::Create)
            .change("contract", (None,approval.token_address))
            .change("owner", (None,approval.owner_address))
            .change("spender", (None,approval.spender_address))
            .change("quantity", (None,approval.quantity))
            .change("trx_hash", (None,approval.trx_hash))
            .change("block_number", (None,approval.block_number))
            .change("block_hash", (None,approval.block_hash))
            .change("block_timestamp", (None,approval.timestamp))
            .change("log_index", (None,approval.log_index))
            .change("transaction_index", (None,approval.transaction_index ));
    }
}