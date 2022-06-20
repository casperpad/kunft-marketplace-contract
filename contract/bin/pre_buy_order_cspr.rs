#![no_main]
#![no_std]

use alloc::string::String;
use casper_contract::{
    contract_api::{account, runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{runtime_args, ContractHash, RuntimeArgs, URef, U256, U512};

extern crate alloc;

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

#[no_mangle]
pub extern "C" fn call() {
    let marketplace_contract: ContractHash = {
        let marketplace_contract_str: String = runtime::get_named_arg("marketplace_contract");
        ContractHash::from_formatted_str(&marketplace_contract_str).unwrap()
    };
    let collection: String = runtime::get_named_arg("collection");
    let token_id: U256 = runtime::get_named_arg("token_id");
    let amount: U512 = runtime::get_named_arg("amount");
    let deposit_purse: URef =
        runtime::call_contract(marketplace_contract, "get_deposit_purse", runtime_args! {});
    let account_purse = account::get_main_purse();
    system::transfer_from_purse_to_purse(account_purse, deposit_purse, amount, None)
        .unwrap_or_revert();
    let _: () = runtime::call_contract(
        marketplace_contract,
        "buy_sell_order_cspr",
        runtime_args! {
          "collection" => collection,
          "token_id" => token_id,
          "amount" => amount
        },
    );
}
