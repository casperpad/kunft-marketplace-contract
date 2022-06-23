#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

use alloc::{collections::BTreeSet, string::String};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, runtime_args, CLType, CLTyped, CLValue, ContractHash,
    ContractPackageHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Group,
    Parameter, RuntimeArgs, URef, U256, U512,
};
use contract_utils::{ContractContext, OnChainContractStorage, ReentrancyGuard};
use kunftmarketplace_contract::{Marketplace, Time};

#[derive(Default)]
struct MarketplaceContract(OnChainContractStorage);

impl ContractContext<OnChainContractStorage> for MarketplaceContract {
    fn storage(&self) -> &OnChainContractStorage {
        &self.0
    }
}

impl Marketplace<OnChainContractStorage> for MarketplaceContract {}
impl ReentrancyGuard<OnChainContractStorage> for MarketplaceContract {}

impl MarketplaceContract {
    fn constructor(&mut self, fee: u8, fee_wallet: AccountHash) {
        Marketplace::init(self, fee, fee_wallet);
        ReentrancyGuard::init(self);
    }
}

#[no_mangle]
pub extern "C" fn constructor() {
    let fee: u8 = runtime::get_named_arg("fee");
    let fee_wallet: AccountHash = {
        let fee_wallet_str: String = runtime::get_named_arg("fee_wallet");
        AccountHash::from_formatted_str(&fee_wallet_str).unwrap()
    };
    MarketplaceContract::default().constructor(fee, fee_wallet);
}

#[no_mangle]
pub extern "C" fn create_sell_order() {
    let seller = runtime::get_caller();
    let start_time: Time = runtime::get_named_arg("start_time");
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let price: U512 = runtime::get_named_arg("price");
    MarketplaceContract::default()
        .create_sell_order(seller, start_time, collection, token_id, price);
}

#[no_mangle]
pub extern "C" fn buy_sell_order_cspr() {
    let caller = runtime::get_caller();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    let amount: U512 = runtime::get_named_arg("amount");
    MarketplaceContract::default().buy_sell_order_cspr(caller, collection, token_id, amount);
}

#[no_mangle]
pub extern "C" fn cancel_sell_order() {
    let caller = runtime::get_caller();
    let collection: ContractHash = {
        let collection_str: String = runtime::get_named_arg("collection");
        ContractHash::from_formatted_str(&collection_str).unwrap()
    };
    let token_id: U256 = runtime::get_named_arg("token_id");
    MarketplaceContract::default().cancel_sell_order(caller, collection, token_id);
}

#[no_mangle]
pub extern "C" fn get_deposit_purse() {
    let purse = MarketplaceContract::default().purse();
    runtime::ret(CLValue::from_t(purse).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn call() {
    let contract_name: String = runtime::get_named_arg("contract_name");
    let fee: u8 = runtime::get_named_arg("fee");
    let fee_wallet: String = runtime::get_named_arg("fee_wallet");
    let (contract_hash, _) = storage::new_contract(
        get_entry_points(),
        None,
        Some(String::from(format!(
            "{}_contract_package_hash",
            contract_name
        ))),
        None,
    );

    let package_hash: ContractPackageHash = ContractPackageHash::new(
        runtime::get_key(&format!("{}_contract_package_hash", contract_name))
            .unwrap_or_revert()
            .into_hash()
            .unwrap_or_revert(),
    );
    let constructor_access: URef =
        storage::create_contract_user_group(package_hash, "constructor", 1, Default::default())
            .unwrap_or_revert()
            .pop()
            .unwrap_or_revert();
    let constructor_args = runtime_args! {
        "fee" => fee,
        "fee_wallet" => fee_wallet
    };
    let _: () = runtime::call_contract(contract_hash, "constructor", constructor_args);

    let mut urefs = BTreeSet::new();
    urefs.insert(constructor_access);
    storage::remove_contract_user_group_urefs(package_hash, "constructor", urefs)
        .unwrap_or_revert();

    runtime::put_key(
        &format!("{}_contract_hash", contract_name),
        contract_hash.into(),
    );
}

fn get_entry_points() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(EntryPoint::new(
        "constructor",
        vec![
            Parameter::new("fee", CLType::U8),
            Parameter::new("fee_wallet", CLType::String),
        ],
        <()>::cl_type(),
        EntryPointAccess::Groups(vec![Group::new("constructor")]),
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "create_sell_order",
        vec![
            Parameter::new("start_time", CLType::U64),
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new("price", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "buy_sell_order_cspr",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
            Parameter::new("amount", CLType::U512),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "cancel_sell_order",
        vec![
            Parameter::new("collection", CLType::String),
            Parameter::new("token_id", CLType::U256),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points.add_entry_point(EntryPoint::new(
        "get_deposit_purse",
        vec![],
        CLType::URef,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    entry_points
}
