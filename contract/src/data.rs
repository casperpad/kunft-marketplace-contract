use alloc::string::String;
use casper_contract::{contract_api::system, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{account::AccountHash, ContractHash, Key, URef, U512};
use contract_utils::{get_key, key_and_value_to_str, set_key, Dict};

use crate::{structs::order::SellOrder, Time, TokenId};

const ORDERS_DICT: &str = "orders";

pub struct SellOrders {
    dict: Dict,
}

impl SellOrders {
    pub fn instance() -> SellOrders {
        SellOrders {
            dict: Dict::instance(ORDERS_DICT),
        }
    }

    pub fn init() {
        Dict::init(ORDERS_DICT)
    }

    fn contract_hash_and_value_to_str(
        &self,
        contract_hash: ContractHash,
        created_time: TokenId,
    ) -> String {
        key_and_value_to_str(&Key::from(contract_hash), &created_time)
    }

    pub fn get(&self, contract_hash: ContractHash, token_id: TokenId) -> SellOrder {
        self.dict
            .get(&self.contract_hash_and_value_to_str(contract_hash, token_id))
            .unwrap_or_revert()
    }

    pub fn set(&self, contract_hash: ContractHash, token_id: TokenId, order: SellOrder) {
        self.dict.set(
            &self.contract_hash_and_value_to_str(contract_hash, token_id),
            order,
        );
    }
    pub fn remove(&self, contract_hash: ContractHash, token_id: TokenId) {
        self.dict
            .remove::<SellOrder>(&self.contract_hash_and_value_to_str(contract_hash, token_id));
    }
}

const PURSE_KEY_NAME: &str = "deposit_purse";
const PURSE_BALANCE_KEY_NAME: &str = "purse_balance";
#[derive(Default)]
pub struct DepositPurse {}

impl DepositPurse {
    pub fn init() {
        let purse = system::create_purse();
        set_key(PURSE_KEY_NAME, purse);
        set_key(PURSE_BALANCE_KEY_NAME, U512::zero());
    }

    pub fn purse() -> URef {
        get_key(PURSE_KEY_NAME).unwrap_or_revert()
    }

    pub fn purse_balance() -> U512 {
        get_key(PURSE_BALANCE_KEY_NAME).unwrap_or_revert()
    }

    pub fn update_purse_balance(balance: U512) {
        set_key(PURSE_BALANCE_KEY_NAME, balance);
    }
}

const FEE_KEY: &str = "fee";

pub fn set_fee(fee: u8) {
    set_key(FEE_KEY, fee);
}

pub fn get_fee() -> u8 {
    get_key(FEE_KEY).unwrap_or_revert()
}

const FEE_WALLET_KEY: &str = "fee_wallet";

pub fn set_fee_wallet(wallet: AccountHash) {
    set_key(FEE_WALLET_KEY, wallet);
}

pub fn get_fee_wallet() -> AccountHash {
    get_key(FEE_WALLET_KEY).unwrap_or_revert()
}
