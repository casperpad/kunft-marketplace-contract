use casper_types::runtime_args;
use test_env::{TestContract, TestEnv};

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
            runtime_args! {},
        ))
    }
}
