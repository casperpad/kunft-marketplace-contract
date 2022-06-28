use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, runtime_args, CLTyped, ContractHash,
    ContractPackageHash, Key, RuntimeArgs, U256,
};
use kunftmarketplace_contract::{Address, Bids, SellOrder, Time, TokenId};
use test_env::{TestContract, TestEnv};

use crate::utils::key_and_value_to_str;

pub struct MarketplaceInstance(TestContract);

impl MarketplaceInstance {
    pub fn new(
        env: &TestEnv,
        contract_name: &str,
        sender: AccountHash,
        fee: u8,
        fee_wallet: Address,
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
        pay_token: Option<String>,
        price: U256,
    ) {
        self.0.call_contract(
            sender,
            "create_sell_order",
            runtime_args! {
                "start_time" => start_time,
                "collection" => collection,
                "token_id" => token_id,
                "pay_token" => pay_token,
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

    pub fn buy_sell_order(&self, sender: AccountHash, collection: String, token_id: U256) {
        self.0.call_contract(
            sender,
            "buy_sell_order",
            runtime_args! {
                "collection" => collection,
                "token_id" => token_id,
            },
        )
    }

    pub fn create_buy_order(
        &self,
        sender: AccountHash,
        collection: String,
        token_id: TokenId,
        additional_recipient: Option<Address>,
        pay_token: String,
        amount: U256,
    ) {
        self.0.call_contract(
            sender,
            "create_buy_order",
            runtime_args! {
                "collection" => collection,
                "token_id" => token_id,
                "additional_recipient" => additional_recipient,
                "pay_token" => pay_token,
                "amount" => amount
            },
        )
    }

    pub fn cancel_buy_order(&self, sender: AccountHash, collection: String, token_id: TokenId) {
        self.0.call_contract(
            sender,
            "cancel_buy_order",
            runtime_args! {
                "collection" => collection,
                "token_id" => token_id,
            },
        )
    }

    pub fn accept_buy_order(
        &self,
        sender: AccountHash,
        collection: String,
        token_id: TokenId,
        bidder: Address,
    ) {
        self.0.call_contract(
            sender,
            "accept_buy_order",
            runtime_args! {
                "collection" => collection,
                "token_id" => token_id,
                "bidder" => bidder
            },
        )
    }

    pub fn result<T: CLTyped + FromBytes>(&self) -> T {
        self.0.query_named_key("result".to_string())
    }

    pub fn buy_order_of(&self, collection: ContractHash, token_id: U256) -> Bids {
        self.0
            .query_dictionary(
                "buy_orders",
                key_and_value_to_str(&Key::from(collection), &token_id),
            )
            .unwrap()
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
