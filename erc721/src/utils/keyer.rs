pub fn token_store_key(token_address: &String, token_id: &String) -> String {
    format!("{}/{}", token_address, token_id)
}

pub fn transfer_key(block_number: u64, log_index: u64) -> String {
    format!("{}/{}", block_number, log_index)
}

pub fn operator_key(trx_hash: &String, log_index: u64) -> String {
    format!("{}/{}", trx_hash,log_index)
}