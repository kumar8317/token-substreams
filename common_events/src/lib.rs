mod abi;

use common::pb::zdexer::eth::events::v1::{OwnershipTransfer,OwnershipTransfers};
use common::format_with_0x;
use substreams::{errors::Error, Hex};
use substreams_ethereum::Event;
use substreams_ethereum::pb::eth::v2 as eth;

#[substreams::handlers::map]
pub fn map_ownership_transfers(blk: eth::Block) -> Result<OwnershipTransfers, Error> {
    
   let ownership_transfers:Vec<OwnershipTransfer> =  blk.receipts().flat_map(|view| {
        view.receipt.logs.iter().filter_map(|log| {
            if let Some(event) = abi::ownable::events::OwnershipTransferred::match_and_decode(log) {
                return Some(
                    OwnershipTransfer {
                        contract_address: format_with_0x(Hex::encode(&log.address).to_string()),
                        previous_owner: format_with_0x(Hex::encode(&event.previous_owner).to_string()),
                        new_owner: format_with_0x(Hex::encode(&event.new_owner).to_string())
                    }
                )
            }
            None
        })
    }).collect();

    Ok(OwnershipTransfers { items: ownership_transfers })
}