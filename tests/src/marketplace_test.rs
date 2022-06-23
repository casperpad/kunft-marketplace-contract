use casper_types::{account::AccountHash, runtime_args, Key, RuntimeArgs, U512};
use std::{collections::BTreeMap, path::PathBuf, vec};
use test_env::{utils::DeploySource, TestEnv};

use crate::{
    cep47_instance::{CEP47Instance, Meta, TokenId},
    marketplace_instance::MarketplaceInstance,
};

const PRE_BUY_ORDER_WASM: &str = "pre_buy_order_cspr.wasm";

struct TestContext {
    marketplace: MarketplaceInstance,
    nft: CEP47Instance,
}

mod meta {
    use super::{BTreeMap, Meta};
    pub fn contract_meta() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("origin".to_string(), "fire".to_string());
        meta
    }

    pub fn red_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "red".to_string());
        meta
    }

    pub fn _blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }

    pub fn _black_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "black".to_string());
        meta
    }

    pub fn _gold_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "gold".to_string());
        meta
    }
}

fn deploy() -> (TestEnv, TestContext, AccountHash) {
    let env = TestEnv::new();
    let owner = env.next_user();

    let marketplace = MarketplaceInstance::new(
        &env,
        "kunft_marketplace",
        owner,
        250u8,
        owner.to_formatted_string(),
    );

    let nft = CEP47Instance::new(&env, "kunft", owner, "KUNFT", "KNFT", meta::contract_meta());

    let test_context = TestContext { marketplace, nft };

    (env, test_context, owner)
}

#[test]
fn should_deploy() {
    let _ = deploy();
}

#[test]
fn should_create_sell_order_and_buy() {
    let (env, test_context, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    let nft = test_context.nft;
    let marketplace = test_context.marketplace;
    nft.mint_one(owner, user, token_id, token_meta.clone());

    nft.approve(
        user,
        Key::from(marketplace.contract_package_hash()),
        vec![token_id],
    );

    let price = U512::one();
    marketplace.create_sell_order(
        user,
        0u64,
        nft.contract_hash().to_formatted_string(),
        token_id,
        price,
    );

    // buy nft
    let buyer = env.next_user();
    let session_code = PathBuf::from(PRE_BUY_ORDER_WASM);
    env.run(
        buyer,
        DeploySource::Code(session_code),
        runtime_args! {
            "marketplace_contract" => marketplace.contract_hash().to_formatted_string(),
            "collection" => nft.contract_hash().to_formatted_string(),
            "token_id" => token_id,
            "amount" => price
        },
    );

    let nft_owner = nft.owner_of(token_id).unwrap();
    assert_eq!(nft_owner, Key::from(buyer));
}

#[test]
fn should_create_sell_order_and_cancel() {
    let (env, test_context, owner) = deploy();
    let user = env.next_user();
    let token_id = TokenId::zero();
    let token_meta = meta::red_dragon();
    let nft = test_context.nft;
    let marketplace = test_context.marketplace;
    nft.mint_one(owner, user, token_id, token_meta.clone());

    nft.approve(
        user,
        Key::from(marketplace.contract_package_hash()),
        vec![token_id],
    );

    let price = U512::one();
    marketplace.create_sell_order(
        user,
        0u64,
        nft.contract_hash().to_formatted_string(),
        token_id,
        price,
    );

    let sell_order = marketplace.sell_order_of(nft.contract_hash(), token_id);

    assert!(sell_order.buyer.is_none());

    marketplace.cancel_sell_order(user, nft.contract_hash().to_formatted_string(), token_id);
}
