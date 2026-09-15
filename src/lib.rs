#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, symbol_short, vec, Vec, Map};

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
    pub status: u32, // 0: active, 1: pending, 2: sold
    pub created_at: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct Offer {
    pub id: String,
    pub listing_id: String,
    pub buyer: Address,
    pub price: i128,
    pub status: u32, // 0: pending, 1: accepted, 2: rejected
    pub created_at: u64,
}

#[contract]
pub struct PropertyRegistry;

#[contractimpl]
impl PropertyRegistry {
    pub fn initialize(env: Env) {
        env.storage().instance().set::<String, u64>(
            &String::from_slice(&env, "version"),
            &2u64,
        );
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
    ) -> Property {
        owner.require_auth();

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

        env.storage()
            .persistent()
            .set::<String, Property>(&id, &property);

        property
    }

    pub fn get_property(env: Env, id: String) -> Option<Property> {
        env.storage().persistent().get::<String, Property>(&id)
    }

    pub fn create_listing(
        env: Env,
        id: String,
        property_id: String,
        seller: Address,
        price: i128,
        currency: String,
    ) -> Listing {
        seller.require_auth();

        let listing = Listing {
            id: id.clone(),
            property_id,
            seller,
            price,
            currency,
            status: 0, // active
            created_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set::<String, Listing>(&id, &listing);

        listing
    }

    pub fn get_listing(env: Env, id: String) -> Option<Listing> {
        env.storage().persistent().get::<String, Listing>(&id)
    }

    pub fn create_offer(
        env: Env,
        id: String,
        listing_id: String,
        buyer: Address,
        price: i128,
    ) -> Offer {
        buyer.require_auth();

        let offer = Offer {
            id: id.clone(),
            listing_id,
            buyer,
            price,
            status: 0, // pending
            created_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set::<String, Offer>(&id, &offer);

        offer
    }

    pub fn get_offer(env: Env, id: String) -> Option<Offer> {
        env.storage().persistent().get::<String, Offer>(&id)
    }

    pub fn accept_offer(env: Env, offer_id: String) -> Offer {
        if let Some(mut offer) = env.storage().persistent().get::<String, Offer>(&offer_id) {
            if let Some(listing) = env.storage()
                .persistent()
                .get::<String, Listing>(&offer.listing_id)
            {
                listing.seller.require_auth();

                offer.status = 1; // accepted
                env.storage()
                    .persistent()
                    .set::<String, Offer>(&offer_id, &offer);

                offer
            } else {
                env.panic_with_error(symbol_short!("nolist"));
                unreachable!()
            }
        } else {
            env.panic_with_error(symbol_short!("nooffer"));
            unreachable!()
        }
    }

    pub fn finalize_sale(
        env: Env,
        offer_id: String,
        listing_id: String,
    ) {
        if let Some(offer) = env.storage().persistent().get::<String, Offer>(&offer_id) {
            if let Some(mut listing) = env.storage()
                .persistent()
                .get::<String, Listing>(&listing_id)
            {
                offer.buyer.require_auth();

                listing.status = 2; // sold
                env.storage()
                    .persistent()
                    .set::<String, Listing>(&listing_id, &listing);
            }
        }
    }
}
