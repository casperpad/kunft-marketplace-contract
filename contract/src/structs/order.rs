use alloc::{collections::BTreeMap, vec::Vec};
use casper_types::{ContractHash, U256};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};

use crate::{Address, Time, TokenId};

#[derive(Clone, Copy, Debug, CLTyped, ToBytes, FromBytes)]
pub struct SellOrder {
    pub creator: Address,
    pub collection: ContractHash,
    pub token_id: TokenId,
    pub pay_token: Option<ContractHash>,
    pub price: U256,
    pub start_time: Time,
}

pub struct BuyOrder {
    pub pay_token: Option<ContractHash>,
    pub price: U256,
    pub start_time: Time,
}

pub type Bids = BTreeMap<Address, BuyOrder>;
