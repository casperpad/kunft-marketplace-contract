use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    account::AccountHash, ContractHash, ContractPackageHash, Key, URef, U256, U512,
};
use contract_utils::{ContractContext, ContractStorage};

use crate::{
    data::{self, DepositPurse, SellOrders},
    interfaces::icep47::ICEP47,
    structs::order::SellOrder,
    Error, Time,
};
pub trait Marketplace<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, fee: u8, fee_wallet: AccountHash) {
        SellOrders::init();
        DepositPurse::init();
        self.set_fee(fee);
        self.set_fee_wallet(fee_wallet);
    }

    fn create_sell_order(
        &mut self,
        seller: AccountHash,
        start_time: Time,
        collection: ContractHash,
        token_id: U256,
        price: U512,
    ) {
        let sell_order: SellOrder = SellOrder {
            seller,
            collection,
            token_id,
            price,
            start_time,
            end_time: Time::default(),
            buyer: Option::<AccountHash>::None,
        };
        let approved = ICEP47::new(collection)
            .get_approved(Key::from(seller), token_id)
            .unwrap_or_revert();
        if !approved.eq(&Key::from(self.contract_package_hash())) {
            self.revert(Error::RequireApprove);
        }
        SellOrders::instance().set(collection, token_id, sell_order);
    }

    fn cancel_sell_order(&mut self, seller: AccountHash, collection: ContractHash, token_id: U256) {
        let order = SellOrders::instance().get(collection, token_id);
        if order.seller.ne(&seller) {
            self.revert(Error::NotOrderCreator);
        }
        if order.buyer.is_some() {
            self.revert(Error::FinishedOrder);
        }
        SellOrders::instance().remove(collection, token_id);
    }

    fn buy_sell_order_cspr(
        &mut self,
        caller: AccountHash,
        collection: ContractHash,
        token_id: U256,
        amount: U512,
    ) {
        self.assert_valid_cspr_transfer(amount);
        let mut order = SellOrders::instance().get(collection, token_id);
        self.assert_order_is_active(&order);
        // Send NFT to caller
        ICEP47::new(order.collection).transfer_from(
            Key::from(order.seller),
            Key::from(caller),
            vec![token_id],
        );
        order.buyer = Some(caller);
        order.end_time = self.current_block_time();
        self.transfer_cspr_with_fee(order.seller, amount);
        SellOrders::instance().set(collection, token_id, order);
    }

    fn transfer_cspr_with_fee(&mut self, account: AccountHash, amount: U512) {
        let fee = U512::from(self.fee());
        let fee_denominator = U512::exp10(4);
        let transfer_amount_to_account = amount
            .checked_mul(fee_denominator.checked_sub(fee).unwrap_or_revert())
            .unwrap_or_revert()
            .checked_div(fee_denominator)
            .unwrap_or_revert();

        let transfer_amount_to_fee_wallet = amount
            .checked_mul(fee)
            .unwrap_or_revert()
            .checked_div(fee_denominator)
            .unwrap_or_revert();
        let fee_wallet = self.fee_wallet();

        self.transfer_cspr(account, transfer_amount_to_account);
        self.transfer_cspr(fee_wallet, transfer_amount_to_fee_wallet);
    }

    fn transfer_cspr(&mut self, account: AccountHash, amount: U512) {
        let purse: URef = DepositPurse::purse();
        system::transfer_from_purse_to_account(purse, account, amount, None).unwrap_or_revert();
        self.update_purse_balance();
    }

    fn update_purse_balance(&mut self) {
        let new_purse_balance = system::get_purse_balance(self.purse()).unwrap_or_default();
        DepositPurse::update_purse_balance(new_purse_balance);
    }

    fn purse(&self) -> URef {
        DepositPurse::purse()
    }

    fn stored_purse_balance(&self) -> U512 {
        DepositPurse::purse_balance()
    }

    fn assert_valid_cspr_transfer(&mut self, amount: U512) {
        let new_purse_balance = system::get_purse_balance(self.purse()).unwrap_or_default();
        let old_purse_balance = self.stored_purse_balance();

        if !old_purse_balance
            .checked_add(amount)
            .unwrap_or_default()
            .eq(&new_purse_balance)
        {
            // entrypoint is called directly
            self.revert(Error::PermissionDenied);
        }
        self.update_purse_balance();
    }

    fn set_fee(&mut self, fee: u8) {
        data::set_fee(fee);
    }

    fn fee(&self) -> u8 {
        data::get_fee()
    }

    fn set_fee_wallet(&mut self, wallet: AccountHash) {
        data::set_fee_wallet(wallet);
    }

    fn fee_wallet(&self) -> AccountHash {
        data::get_fee_wallet()
    }

    fn _check_offer_is_acceptable(&self) {}

    fn current_block_time(&self) -> u64 {
        u64::from(runtime::get_blocktime())
    }

    fn revert(&self, error: Error) {
        runtime::revert(error);
    }

    fn assert_order_is_active(&self, order: &SellOrder) {
        if order.buyer.is_some() {
            self.revert(Error::FinishedOrder);
        }
    }

    fn contract_package_hash(&self) -> ContractPackageHash {
        let hash_addr = self.self_addr().into_hash().unwrap();
        ContractPackageHash::from(hash_addr)
    }
}
