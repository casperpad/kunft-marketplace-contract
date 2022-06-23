use alloc::vec::Vec;
use casper_types::{account::AccountHash, ContractHash, U512};
use casper_types_derive::{CLTyped, FromBytes, ToBytes};

use crate::{Time, TokenId};

#[derive(Clone, Copy, Debug, CLTyped, ToBytes, FromBytes)]
pub struct SellOrder {
    pub seller: AccountHash,
    pub collection: ContractHash,
    pub token_id: TokenId,
    pub price: U512,
    pub start_time: Time,
    pub end_time: Time,
    pub buyer: Option<AccountHash>,
}
