#![no_std]

mod errors;
mod events;
mod fractional;
mod storage;

use errors::Error;
use storage::{
    DataKey, LISTING_ACTIVE, LISTING_PENDING, LISTING_SOLD, OFFER_ACCEPTED, OFFER_PENDING,
    OFFER_REJECTED,
};
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

pub use errors::Error as RegistryError;
pub use fractional::{cost, ShareLot};
pub use storage::VERSION;

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Property {
    pub id: String,
    pub title: String,
    pub location: String,
    pub price: i128,
    pub currency: String,
    pub owner: Address,
    pub nft_contract: Address,
    pub nft_id: String,
    pub metadata_uri: String,
    pub created_at: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Listing {
    pub id: String,
    pub property_id: String,
    pub seller: Address,
    pub price: i128,
    pub currency: String,
    pub status: u32,
    pub created_at: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Offer {
    pub id: String,
    pub listing_id: String,
    pub buyer: Address,
    pub price: i128,
    pub status: u32,
    pub created_at: u64,
}

fn require_unpaused(env: &Env) -> Result<(), Error> {
    let paused: bool = env
        .storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false);
    if paused {
        Err(Error::Paused)
    } else {
        Ok(())
    }
}

fn require_admin(env: &Env) -> Result<Address, Error> {
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotFound)?;
    admin.require_auth();
    Ok(admin)
}

fn require_positive_price(price: i128) -> Result<(), Error> {
    if price <= 0 {
        Err(Error::ZeroPrice)
    } else {
        Ok(())
    }
}

#[contract]
pub struct PropertyRegistry;

#[contractimpl]
impl PropertyRegistry {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Version, &VERSION);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().extend_ttl(100_000, 100_000);
        Ok(())
    }

    pub fn version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Version)
            .unwrap_or(0)
    }

    pub fn admin(env: Env) -> Option<Address> {
        env.storage().instance().get(&DataKey::Admin)
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }

    pub fn pause(env: Env) -> Result<(), Error> {
        let admin = require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &true);
        events::pause(&env, &admin, true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), Error> {
        let admin = require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &false);
        events::pause(&env, &admin, false);
        Ok(())
    }

    pub fn register_property(
        env: Env,
        id: String,
        title: String,
        location: String,
        price: i128,
        currency: String,
        owner: Address,
        nft_contract: Address,
        nft_id: String,
        metadata_uri: String,
    ) -> Result<Property, Error> {
        require_unpaused(&env)?;
        require_positive_price(price)?;
        owner.require_auth();

        let key = DataKey::Property(id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadyExists);
        }

        let property = Property {
            id: id.clone(),
            title,
            location,
            price,
            currency,
            owner: owner.clone(),
            nft_contract,
            nft_id,
            metadata_uri,
            created_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &property);
        env.storage().persistent().extend_ttl(&key, 50_000, 50_000);
        events::register(&env, &owner, &id, price);
        Ok(property)
    }

    pub fn get_property(env: Env, id: String) -> Option<Property> {
        env.storage().persistent().get(&DataKey::Property(id))
    }

    pub fn create_listing(
        env: Env,
        id: String,
        property_id: String,
        seller: Address,
        price: i128,
        currency: String,
    ) -> Result<Listing, Error> {
        require_unpaused(&env)?;
        require_positive_price(price)?;
        seller.require_auth();

        let property: Property = env
            .storage()
            .persistent()
            .get(&DataKey::Property(property_id.clone()))
            .ok_or(Error::NotFound)?;
        if property.owner != seller {
            return Err(Error::Unauthorized);
        }

        let key = DataKey::Listing(id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadyExists);
        }

        let listing = Listing {
            id: id.clone(),
            property_id,
            seller: seller.clone(),
            price,
            currency,
            status: LISTING_ACTIVE,
            created_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &listing);
        env.storage().persistent().extend_ttl(&key, 50_000, 50_000);
        events::list(&env, &seller, &id, price);
        Ok(listing)
    }

    pub fn get_listing(env: Env, id: String) -> Option<Listing> {
        env.storage().persistent().get(&DataKey::Listing(id))
    }

    pub fn cancel_listing(env: Env, id: String) -> Result<Listing, Error> {
        require_unpaused(&env)?;
        let key = DataKey::Listing(id.clone());
        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&key)
            .ok_or(Error::NotFound)?;
        listing.seller.require_auth();
        if listing.status != LISTING_ACTIVE {
            return Err(Error::InvalidState);
        }
        listing.status = LISTING_SOLD;
        env.storage().persistent().set(&key, &listing);
        events::cancel(&env, &listing.seller, &id);
        Ok(listing)
    }

    pub fn create_offer(
        env: Env,
        id: String,
        listing_id: String,
        buyer: Address,
        price: i128,
    ) -> Result<Offer, Error> {
        require_unpaused(&env)?;
        require_positive_price(price)?;
        buyer.require_auth();

        let listing: Listing = env
            .storage()
            .persistent()
            .get(&DataKey::Listing(listing_id.clone()))
            .ok_or(Error::NotFound)?;
        if listing.status == LISTING_SOLD {
            return Err(Error::Sold);
        }
        if listing.status != LISTING_ACTIVE {
            return Err(Error::ListingNotActive);
        }

        let key = DataKey::Offer(id.clone());
        if env.storage().persistent().has(&key) {
            return Err(Error::AlreadyExists);
        }

        let offer = Offer {
            id: id.clone(),
            listing_id,
            buyer: buyer.clone(),
            price,
            status: OFFER_PENDING,
            created_at: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&key, &offer);
        env.storage().persistent().extend_ttl(&key, 50_000, 50_000);
        events::offer(&env, &buyer, &id, price);
        Ok(offer)
    }

    pub fn get_offer(env: Env, id: String) -> Option<Offer> {
        env.storage().persistent().get(&DataKey::Offer(id))
    }

    pub fn accept_offer(env: Env, offer_id: String) -> Result<Offer, Error> {
        require_unpaused(&env)?;
        let offer_key = DataKey::Offer(offer_id.clone());
        let mut offer: Offer = env
            .storage()
            .persistent()
            .get(&offer_key)
            .ok_or(Error::NotFound)?;
        if offer.status != OFFER_PENDING {
            return Err(Error::OfferNotPending);
        }

        let listing_key = DataKey::Listing(offer.listing_id.clone());
        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&listing_key)
            .ok_or(Error::NotFound)?;
        listing.seller.require_auth();
        if listing.status != LISTING_ACTIVE {
            return Err(Error::ListingNotActive);
        }

        offer.status = OFFER_ACCEPTED;
        listing.status = LISTING_PENDING;
        env.storage().persistent().set(&offer_key, &offer);
        env.storage().persistent().set(&listing_key, &listing);
        events::accept(&env, &listing.seller, &offer_id);
        Ok(offer)
    }

    pub fn reject_offer(env: Env, offer_id: String) -> Result<Offer, Error> {
        require_unpaused(&env)?;
        let offer_key = DataKey::Offer(offer_id.clone());
        let mut offer: Offer = env
            .storage()
            .persistent()
            .get(&offer_key)
            .ok_or(Error::NotFound)?;
        if offer.status != OFFER_PENDING {
            return Err(Error::OfferNotPending);
        }

        let listing: Listing = env
            .storage()
            .persistent()
            .get(&DataKey::Listing(offer.listing_id.clone()))
            .ok_or(Error::NotFound)?;
        listing.seller.require_auth();

        offer.status = OFFER_REJECTED;
        env.storage().persistent().set(&offer_key, &offer);
        events::reject(&env, &listing.seller, &offer_id);
        Ok(offer)
    }

    pub fn finalize_sale(env: Env, offer_id: String, listing_id: String) -> Result<Listing, Error> {
        require_unpaused(&env)?;
        let offer: Offer = env
            .storage()
            .persistent()
            .get(&DataKey::Offer(offer_id))
            .ok_or(Error::NotFound)?;
        if offer.status != OFFER_ACCEPTED {
            return Err(Error::InvalidState);
        }
        if offer.listing_id != listing_id {
            return Err(Error::InvalidState);
        }

        let listing_key = DataKey::Listing(listing_id.clone());
        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&listing_key)
            .ok_or(Error::NotFound)?;
        if listing.status != LISTING_PENDING {
            return Err(Error::InvalidState);
        }

        offer.buyer.require_auth();

        listing.status = LISTING_SOLD;
        env.storage().persistent().set(&listing_key, &listing);

        let property_key = DataKey::Property(listing.property_id.clone());
        let mut property: Property = env
            .storage()
            .persistent()
            .get(&property_key)
            .ok_or(Error::NotFound)?;
        property.owner = offer.buyer.clone();
        env.storage().persistent().set(&property_key, &property);

        events::finalize(&env, &offer.buyer, &listing_id);
        Ok(listing)
    }
}
