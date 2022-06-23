use casper_types::{
    account::AccountHash, runtime_args, ContractHash, ContractPackageHash, Key, RuntimeArgs, U256,
    U512,
};
use kunftmarketplace_contract::{SellOrder, Time};
use test_env::{TestContract, TestEnv};

use crate::utils::key_and_value_to_str;

pub struct MarketplaceInstance(TestContract);

impl MarketplaceInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        fee: u8,
        fee_wallet: String,
    ) -> MarketplaceInstance {
        MarketplaceInstance(TestContract::new(
            env,
            "marketplace_contract.wasm",
            contract_name,
            sender,
            runtime_args! {
                "fee" => fee,
                "fee_wallet" => fee_wallet
            },
        ))
    }

    pub fn create_sell_order(
        &self,
        sender: AccountHash,
        start_time: Time,
        collection: String,
        token_id: U256,
        price: U512,
    ) {
        self.0.call_contract(
            sender,
            "create_sell_order",
            runtime_args! {
                "start_time" => start_time,
                "collection" => collection,
                "token_id" => token_id,
                "price" => price,
            },
        )
    }

    pub fn cancel_sell_order(&self, sender: AccountHash, collection: String, token_id: U256) {
        self.0.call_contract(
            sender,
            "cancel_sell_order",
            runtime_args! {
                "collection" => collection,
                "token_id" => token_id,
            },
        )
    }

    pub fn sell_order_of(&self, collection: ContractHash, token_id: U256) -> SellOrder {
        self.0
            .query_dictionary(
                "sell_orders",
                key_and_value_to_str(&Key::from(collection), &token_id),
            )
            .unwrap()
    }

    pub fn contract_package_hash(&self) -> ContractPackageHash {
        self.0.contract_package_hash()
    }

    pub fn contract_hash(&self) -> ContractHash {
        self.0.contract_hash()
    }
}
