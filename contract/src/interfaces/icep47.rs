#![allow(dead_code)]
use alloc::vec::Vec;
use casper_contract::contract_api::runtime;
use casper_types::{runtime_args, ContractHash, Key, RuntimeArgs, U256};

use crate::TokenId;

pub struct ICEP47 {
    pub contract_hash: ContractHash,
}

impl ICEP47 {
    pub fn new(contract_hash: ContractHash) -> Self {
        ICEP47 { contract_hash }
    }

    pub fn balance_of(&self, owner: Key) -> U256 {
        runtime::call_contract(
            self.contract_hash,
            "balance_of",
            runtime_args! {
              "owner" => owner,
            },
        )
    }

    pub fn approve(&self, spender: Key, token_ids: Vec<U256>) {
        runtime::call_contract::<()>(
            self.contract_hash,
            "approve",
            runtime_args! {
              "spender" => spender,
              "token_ids" => token_ids
            },
        );
    }
    pub fn get_approved(&self, owner: Key, token_id: TokenId) -> Option<Key> {
        runtime::call_contract(
            self.contract_hash,
            "get_approved",
            runtime_args! {
              "owner" => owner,
              "token_id" => token_id
            },
        )
    }

    pub fn owner_of(&self, token_id: TokenId) -> Option<Key> {
        runtime::call_contract(
            self.contract_hash,
            "owner_of",
            runtime_args! {
              "token_id" => token_id
            },
        )
    }

    pub fn transfer(&self, recipient: Key, token_ids: Vec<U256>) {
        runtime::call_contract::<()>(
            self.contract_hash,
            "transfer",
            runtime_args! {
              "recipient" => recipient,
              "token_ids" => token_ids
            },
        );
    }

    pub fn transfer_from(&self, sender: Key, recipient: Key, token_ids: Vec<U256>) {
        runtime::call_contract::<()>(
            self.contract_hash,
            "transfer_from",
            runtime_args! {
              "sender" => sender,
              "recipient" => recipient,
              "token_ids" => token_ids
            },
        );
    }
}
