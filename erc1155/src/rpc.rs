use substreams_ethereum::{ pb::eth::{rpc::RpcResponse } };
use crate::abi::erc1155;
use substreams_ethereum::rpc::RpcBatch;
#[derive(Debug)]

pub struct RpcTokenURI {
    pub to: Vec<u8>,
    pub tokenid: substreams::scalar::BigInt,
}

pub fn fetch_token_uri(params: Vec<RpcTokenURI>) -> Vec<RpcResponse> {
    let mut response = Vec::new();
    let mut batch = RpcBatch::new();

    for (i, call) in params.iter().enumerate() {
        batch = batch.add(
            erc1155::functions::Uri { param0: call.tokenid.clone() },
            call.to.clone()
        );

        if (i + 1) % 50 == 0 || i == params.len() - 1 {
            let batch_response = batch.execute().unwrap().responses;
            response.extend(batch_response);

            batch = RpcBatch::new();
        }
    }

    response
}
