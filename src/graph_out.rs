use std::str::FromStr;

use substreams::{scalar::BigInt, store::{Deltas, DeltaBigInt}};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};
use common::pb::zdexer::eth::events::v1::OwnershipTransfers;
use crate::{
    pb::zdexer::eth::erc20::v1::{Contracts, Approvals, Transfers},
    utils::{
        keyer::{operator_key, transfer_key},
    },
};

pub fn contract_entity_changes(changes: &mut EntityChanges, contracts: Contracts) {
    for contract in contracts.items {
        account_create_entity_changes(changes, contract.owner_address.clone());
        let key = contract.token_address.clone();
        changes
            .push_change("ERC20Contract", &key, 1, Operation::Create)
            .change("id", contract.token_address)
            .change("owner_address", contract.owner_address)
            .change("name", contract.name)
            .change("symbol", contract.symbol)
            .change("decimals", contract.decimals);
    }
}

pub fn contract_ownership_update_entity_change(changes: &mut EntityChanges, ownership_transfers: OwnershipTransfers) {
    for ownership_transfer in ownership_transfers.items {
        account_create_entity_changes(changes, ownership_transfer.new_owner.clone());
        changes
            .push_change("ERC20Contract", &ownership_transfer.contract_address, 1, Operation::Update)
            .change("owner_address", ownership_transfer.new_owner);
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
            .change("transaction_type", transfer.transaction_type);
    }
}


pub fn approval_entity_changes(changes: &mut EntityChanges, approvals: Approvals) {
    for approval in approvals.items {
        account_create_entity_changes(changes, approval.owner_address.clone());
        account_create_entity_changes(changes, approval.spender_address.clone());
        let key = operator_key(&approval.spender_address, &approval.token_address, &approval.trx_hash, &approval.owner_address);
        changes
            .push_change("ERC20Approval", &key, 1, Operation::Create)
            .change("id", key)
            .change("contract", approval.token_address)
            .change("owner", approval.owner_address)
            .change("spender", approval.spender_address)
            .change("quantity", BigInt::from_str(&approval.quantity).unwrap());
    }
}

pub fn balance_entity_changes(
    changes:&mut EntityChanges,
    deltas: Deltas<DeltaBigInt>
){
    use substreams::pb::substreams::store_delta::Operation as OperationDelta;

    for delta in deltas.deltas {
        let keyclone=delta.key.clone();
        let account_address = keyclone.as_str().split('/').next().unwrap().to_string();
        let token_address = keyclone.as_str().split('/').nth(1).unwrap().to_string();

        account_create_entity_changes(changes, account_address.clone());

        match delta.operation {
            OperationDelta::Create =>{
                changes
                    .push_change("ERC20Balance", &delta.key, delta.ordinal, Operation::Create)
                    .change("id", delta.key)
                    .change("contract", token_address.clone())
                    .change("account", account_address)
                    .change("quantity", delta.new_value);
            },
            OperationDelta::Update =>{
                changes
                    .push_change("ERC20Balance", &delta.key, delta.ordinal, Operation::Update)
                    .change("quantity", (delta.old_value,delta.new_value));
            }
            x=> panic!("unsupported operation {:?}",x),
        }
    }
}
