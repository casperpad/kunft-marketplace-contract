#![no_std]
#[macro_use]
extern crate alloc;

mod data;
mod error;
mod event;
mod interfaces;
mod libs;
mod marketplace;
mod structs;

use casper_types::U256;
pub use error::Error;
pub type Time = u64;
pub type TokenId = U256;
pub use marketplace::Marketplace;
pub use structs::order::SellOrder;
