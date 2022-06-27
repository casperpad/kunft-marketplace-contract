use casper_types::{account::AccountHash, Key};

pub enum MarketplaceEvent {
    SellOrderCreated { seller: Key },
}
