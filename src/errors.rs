use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    Unauthorized = 2,
    InvalidState = 3,
    AlreadyExists = 4,
    Sold = 5,
    ZeroPrice = 6,
    Paused = 7,
    NotAdmin = 8,
    AlreadyInitialized = 9,
    ListingNotActive = 10,
    OfferNotPending = 11,
}
