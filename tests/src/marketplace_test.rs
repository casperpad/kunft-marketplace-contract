use test_env::TestEnv;

use crate::marketplace_instance::MarketplaceInstance;

const NAME: &str = "DragonsNFT";
const SYMBOL: &str = "DGNFT";


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

    pub fn blue_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "blue".to_string());
        meta
    }

    pub fn black_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "black".to_string());
        meta
    }

    pub fn gold_dragon() -> Meta {
        let mut meta = BTreeMap::new();
        meta.insert("color".to_string(), "gold".to_string());
        meta
    }

    pub marketplace_instance: MarketplaceInstance,
    pub nft: CEP47Instance,
}

// fn deploy() -> (TestEnv, TestContext, AccountHash) {
//     let env = TestEnv::new();
//     let owner = env.next_user();

//     let marketplace_instance = MarketplaceInstance::new(&env, "marketplace", owner, 250u8, owner);
//     (env, token, owner)
// }

// #[test]
// fn test_deploy() {
//     let _ = deploy();
// }

// #[test]
// fn should_create_sell_order() {
//     let (env, markteplace, owner) = deploy();
// }
