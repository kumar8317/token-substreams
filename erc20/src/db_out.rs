use substreams::store::{Deltas, DeltaBigInt};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};
use common::sanitize_string;
use crate::{pb::zdexer::eth::erc20::v1::{Transfers, Approvals, Contracts}, utils::keyer::{transfer_key, operator_key}};

// pub fn contract_db_changes(
//     changes:&mut DatabaseChanges,
//     contracts: Contracts
// ) {
//     for contract in contracts.items {
//         //account_db_changes(changes, contract.owner_address.clone());
//         let key = contract.token_address.clone();
//         changes
//             .push_change("erc20_contract", &key, 1, Operation::Create)
//             .change("owner_address", (None,contract.owner_address))
//             .change("name", (None,sanitize_string(&contract.name)))
//             .change("symbol", (None,sanitize_string(&contract.symbol)))
//             .change("decimals", (None,contract.decimals));
//     }
// }

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

// pub fn contract_ownership_update_db_changes(changes: &mut DatabaseChanges, ownership_transfers: OwnershipTransfers) {
//     for ownership_transfer in ownership_transfers.items {
//         //account_db_changes(changes, ownership_transfer.new_owner.clone());
//         changes
//             .push_change("erc20_contract", &ownership_transfer.contract_address, 1, Operation::Update)
//             .change("owner_address", (ownership_transfer.previous_owner,ownership_transfer.new_owner));
//     }
// }

// pub fn account_db_changes(
//     changes:&mut DatabaseChanges,
//     account_address: String
// ) {
//     changes
//         .push_change("account", &account_address, 1, Operation::Create);
// }

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
            .change("transaction_index", (None,transfer.transaction_index))
            .change("transaction_type", (None,transfer.transaction_type));
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
            .change("quantity", (None,approval.quantity));
    }
}

pub fn balance_db_changes(
    changes:&mut DatabaseChanges,
    deltas: Deltas<DeltaBigInt>
) {
    use substreams::pb::substreams::store_delta::Operation as OperationDelta;

    for delta in deltas.deltas {
        let keyclone = delta.key.clone();
        let account_address = keyclone.as_str().split('/').next().unwrap().to_string();
        let token_address = keyclone.as_str().split('/').nth(1).unwrap().to_string();

        //account_db_changes(changes, account_address.clone());

        match delta.operation {
            OperationDelta::Create =>{
                changes
                    .push_change("erc20_balance", &delta.key, delta.ordinal, Operation::Create)
                    .change("contract", (None,token_address.clone()))
                    .change("account", (None,account_address))
                    .change("quantity", (None,delta.new_value.to_string()));
            },
            OperationDelta::Update => {
                changes
                    .push_change("erc20_balance", &delta.key, delta.ordinal, Operation::Update)
                    .change("quantity", (delta.old_value.to_string(),delta.new_value.to_string()));
            },
            x => panic!("unsupported operation {:?}",x),
        }
    }
}