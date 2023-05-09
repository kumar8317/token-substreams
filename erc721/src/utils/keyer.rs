pub fn token_store_key(token_address: &String, token_id: &String) -> String {
    format!("{}/{}", token_address, token_id)
}

pub fn transfer_key(block_number: u64, log_index: u64) -> String {
    format!("{}/{}", block_number, log_index)
}

pub fn operator_key(operator_address:  &String, token_address:  &String, trx_hash: &String, owner_address: &String) -> String {
    format!("{}/{}/{}/{}", trx_hash,operator_address, owner_address, token_address)
}

pub fn approval_key(operator_address:  &String, token_address:  &String, token_id: &String, trx_hash: &String ,owner_address: &String) -> String {
    format!("{}/{}/{}/{}/{}", trx_hash,owner_address, operator_address, token_address, token_id)
}