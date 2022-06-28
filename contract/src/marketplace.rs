use alloc::vec;
use casper_contract::{
    contract_api::{runtime, system},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, CLTyped, ContractHash, ContractPackageHash, RuntimeArgs,
    URef, U256, U512,
};
use contract_utils::{set_key, ContractContext, ContractStorage};

use crate::{
    data::{self, BuyOrders, DepositPurse, SellOrders},
    event::MarketplaceEvent,
    interfaces::{icep47::ICEP47, ierc20::IERC20},
    libs::{u256_to_512, u512_to_u256},
    structs::order::{BuyOrder, SellOrder},
    Address, Error, Time, TokenId,
};
pub trait Marketplace<Storage: ContractStorage>: ContractContext<Storage> {
    fn init(&mut self, fee: u8, fee_wallet: Address) {
        SellOrders::init();
        BuyOrders::init();
        DepositPurse::init();
        self.set_fee(fee);
        self.set_fee_wallet(fee_wallet);
    }

    fn create_sell_order(
        &mut self,
        caller: Address,
        start_time: Time,
        collection: ContractHash,
        token_id: TokenId,
        pay_token: Option<ContractHash>,
        price: U256,
    ) {
        let sell_order: SellOrder = SellOrder {
            creator: caller,
            collection,
            token_id,
            pay_token,
            price,
            start_time,
        };

        let approved = ICEP47::new(collection)
            .get_approved(caller, token_id)
            .unwrap_or_revert_with(Error::RequireApprove);

        if !approved.eq(&Address::from(self.contract_package_hash())) {
            self.revert(Error::RequireApprove);
        }
        ICEP47::new(collection).transfer_from(
            caller,
            Address::from(self.contract_package_hash()),
            vec![token_id],
        );
        SellOrders::instance().set(collection, token_id, sell_order);
    }

    fn cancel_sell_order(&mut self, caller: Address, collection: ContractHash, token_id: TokenId) {
        let order = SellOrders::instance().get(collection, token_id);
        if order.creator.ne(&caller) {
            self.revert(Error::NotOrderCreator);
        }
        self.assert_order_is_active(&order);
        ICEP47::new(collection).transfer(caller, vec![token_id]);
        SellOrders::instance().remove(collection, token_id);
    }

    fn buy_sell_order_cspr(
        &mut self,
        caller: Address,
        collection: ContractHash,
        token_id: TokenId,
        amount: U512,
        addtional_recipient: Option<Address>,
    ) {
        self.assert_valid_cspr_transfer(amount);
        let order = SellOrders::instance().get(collection, token_id);
        self.assert_order_is_active(&order);
        if order.pay_token.is_some() {
            self.revert(Error::InvalidPayToken);
        }
        let amount_u256 = u512_to_u256(&amount).unwrap();
        if amount_u256.lt(&order.price) {
            self.revert(Error::InsufficientBalance);
        }

        // Send NFT
        match addtional_recipient {
            Some(address) => {
                ICEP47::new(order.collection).transfer(Address::from(address), vec![token_id]);
            }
            None => {
                ICEP47::new(order.collection).transfer(Address::from(caller), vec![token_id]);
            }
        };

        self.transfer_cspr_with_fee(order.creator, amount);
        SellOrders::instance().set(collection, token_id, order);
    }

    fn buy_sell_order(
        &mut self,
        caller: Address,
        collection: ContractHash,
        token_id: TokenId,
        amount: U256,
        addtional_recipient: Option<Address>,
    ) {
        let order = SellOrders::instance().get(collection, token_id);
        self.assert_order_is_active(&order);
        if order.pay_token.is_none() {
            self.revert(Error::InvalidPayToken);
        }
        let allowance = IERC20::new(order.pay_token.unwrap()).allowance(
            Address::from(caller),
            Address::from(self.contract_package_hash()),
        );
        if allowance.lt(&amount) {
            self.revert(Error::InsufficientBalance);
        }

        // Transfer pay token
        self.transfer_with_fee(
            Some(Address::from(caller)),
            order.creator,
            order.pay_token.unwrap(),
            amount,
        );

        // Send NFT
        match addtional_recipient {
            Some(address) => {
                ICEP47::new(order.collection).transfer(Address::from(address), vec![token_id]);
            }
            None => {
                ICEP47::new(order.collection).transfer(Address::from(caller), vec![token_id]);
            }
        };

        SellOrders::instance().set(collection, token_id, order);
    }

    fn create_buy_order_cspr(
        &mut self,
        caller: Address,
        collection: ContractHash,
        token_id: TokenId,
        additional_recipient: Option<Address>,
        amount: U512,
    ) {
        self.assert_valid_cspr_transfer(amount);
        let mut bids = BuyOrders::instance().get(collection, token_id);

        if bids.contains_key(&caller) {
            self.revert(Error::AlreadyExistOrder);
        }

        let buy_order = BuyOrder {
            pay_token: None,
            price: u512_to_u256(&amount).unwrap(),
            start_time: self.current_block_time(),
            additional_recipient,
        };
        bids.insert(caller, buy_order);
        BuyOrders::instance().set(collection, token_id, bids);
    }

    fn create_buy_order(
        &mut self,
        caller: Address,
        collection: ContractHash,
        token_id: TokenId,
        additional_recipient: Option<Address>,
        pay_token: ContractHash,
        amount: U256,
    ) {
        let mut bids = BuyOrders::instance().get(collection, token_id);

        if bids.contains_key(&caller) {
            self.revert(Error::AlreadyExistOrder);
        }
        let allowance = IERC20::new(pay_token).allowance(
            Address::from(caller),
            Address::from(self.contract_package_hash()),
        );
        if allowance.lt(&amount) {
            self.revert(Error::InsufficientBalance);
        }
        IERC20::new(pay_token).transfer_from(
            caller,
            Address::from(self.contract_package_hash()),
            amount,
        );
        let buy_order = BuyOrder {
            pay_token: Some(pay_token),
            price: amount,
            start_time: self.current_block_time(),
            additional_recipient,
        };
        bids.insert(caller, buy_order);
        BuyOrders::instance().set(collection, token_id, bids);
    }

    fn cancel_buy_order(&mut self, caller: Address, collection: ContractHash, token_id: TokenId) {
        let mut bids = BuyOrders::instance().get(collection, token_id);

        match bids.get(&caller) {
            Some(bid) => match bid.pay_token {
                Some(contract_hash) => {
                    IERC20::new(contract_hash).transfer(caller, bid.price);
                }
                None => {
                    self.transfer_cspr(caller, u256_to_512(&bid.price).unwrap());
                }
            },
            None => {
                self.revert(Error::NotExistOrder);
            }
        }
        bids.remove(&caller);
        BuyOrders::instance().set(collection, token_id, bids);
    }

    fn accept_buy_order(
        &mut self,
        caller: Address,
        collection: ContractHash,
        token_id: TokenId,
        bidder: Address,
    ) {
        let token_owner = ICEP47::new(collection)
            .owner_of(token_id)
            .unwrap_or_revert_with(Error::NotExistToken);
        if caller.ne(&token_owner) {
            self.revert(Error::NotTokenOwner);
        }

        let approved = ICEP47::new(collection)
            .get_approved(caller, token_id)
            .unwrap_or_revert_with(Error::RequireApprove);

        if !approved.eq(&Address::from(self.contract_package_hash())) {
            self.revert(Error::RequireApprove);
        }

        let mut bids = BuyOrders::instance().get(collection, token_id);

        match bids.get(&bidder) {
            Some(bid) => {
                let to = match bid.additional_recipient {
                    Some(address) => address,
                    None => caller,
                };
                match bid.pay_token {
                    Some(contract_hash) => {
                        self.transfer_with_fee(None, to, contract_hash, bid.price)
                    }
                    None => {
                        self.transfer_cspr_with_fee(to, u256_to_512(&bid.price).unwrap());
                    }
                }
            }
            None => {
                self.revert(Error::NotExistOrder);
            }
        }
        ICEP47::new(collection).transfer_from(caller, bidder, vec![token_id]);
        bids.remove(&bidder);
        BuyOrders::instance().set(collection, token_id, bids);
    }

    fn transfer_with_fee(
        &self,
        from: Option<Address>,
        to: Address,
        contract_hash: ContractHash,
        amount: U256,
    ) {
        let fee = U256::from(self.fee());
        let fee_denominator = U256::exp10(4);
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
        match from {
            Some(address) => {
                IERC20::new(contract_hash).transfer_from(address, to, transfer_amount_to_account);
                IERC20::new(contract_hash).transfer_from(
                    address,
                    fee_wallet,
                    transfer_amount_to_fee_wallet,
                );
            }
            None => {
                IERC20::new(contract_hash).transfer(to, transfer_amount_to_account);
                IERC20::new(contract_hash).transfer(fee_wallet, transfer_amount_to_fee_wallet);
            }
        }
    }

    fn transfer_cspr_with_fee(&mut self, account: Address, amount: U512) {
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

    fn transfer_cspr(&mut self, account: Address, amount: U512) {
        let purse: URef = DepositPurse::purse();
        match account {
            Address::Account(account_hash) => {
                system::transfer_from_purse_to_account(purse, account_hash, amount, None)
                    .unwrap_or_revert();
            }
            Address::Contract(package_hash) => {
                let target_purse = runtime::call_versioned_contract(
                    package_hash,
                    None,
                    "get_purse",
                    runtime_args! {},
                );
                system::transfer_from_purse_to_purse(purse, target_purse, amount, None)
                    .unwrap_or_revert();
            }
        };

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

    fn set_fee_wallet(&mut self, wallet: Address) {
        data::set_fee_wallet(wallet);
    }

    fn fee_wallet(&self) -> Address {
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
        let token_owner = ICEP47::new(order.collection)
            .owner_of(order.token_id)
            .unwrap();
        if token_owner.ne(&Address::from(self.contract_package_hash())) {
            runtime::revert(Error::FinishedOrder);
        }
    }

    fn store_result<T: CLTyped + ToBytes>(&mut self, value: T) {
        set_key("result", value);
    }

    fn contract_package_hash(&self) -> ContractPackageHash {
        let hash_addr = self.self_addr().into_hash().unwrap();
        ContractPackageHash::from(hash_addr)
    }
    fn emit(&mut self, event: MarketplaceEvent) {
        data::emit(&event, self.contract_package_hash());
    }
}
