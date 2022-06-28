use casper_types::{ContractHash, U256};

use crate::{Address, TokenId};

pub enum MarketplaceEvent {
    SellOrderCreated {
        creator: Address,
        collection: ContractHash,
        token_id: TokenId,
        pay_token: Option<ContractHash>,
        price: U256,
    },
}
