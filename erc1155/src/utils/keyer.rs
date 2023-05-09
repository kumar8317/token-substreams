pub fn token_store_key(token_address: &String, token_id: &String) -> String {
    format!("{}/{}", token_address, token_id)
}

pub fn transfer_key(block_number: u64, log_index: u64, token_id: &String) -> String {
    format!("{}/{}/{}", block_number, log_index, token_id)
}

pub fn operator_key(operator_address:  &String, token_address:  &String, trx_hash: &String, owner_address: &String) -> String {
    format!("{}/{}/{}/{}", trx_hash,operator_address, owner_address, token_address)
}

pub fn balance_key(
    account_address: &String,
    contract_address: &String,
    token_id: &String,
) -> String {
    format!("{}/{}/{}", account_address, contract_address, token_id,)
}