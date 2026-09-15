use soroban_sdk::{contracttype, String};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Version,
    Admin,
    Paused,
    Property(String),
    Listing(String),
    Offer(String),
}

pub const VERSION: u32 = 3;
pub const LISTING_ACTIVE: u32 = 0;
pub const LISTING_PENDING: u32 = 1;
pub const LISTING_SOLD: u32 = 2;
pub const OFFER_PENDING: u32 = 0;
pub const OFFER_ACCEPTED: u32 = 1;
pub const OFFER_REJECTED: u32 = 2;
