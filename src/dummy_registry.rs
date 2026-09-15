#![allow(dead_code)]

/// Off-chain dummy records mirroring on-chain Property/Listing/Offer shapes.
pub struct DummyProperty {
    pub id: &'static str,
    pub title: &'static str,
    pub price: i128,
}

pub const DUMMY_PROPERTIES: &[DummyProperty] = &[
    DummyProperty { id: "prop_001", title: "Sunny Beachfront Villa", price: 500_000 },
    DummyProperty { id: "prop_002", title: "Mountain Retreat Home", price: 750_000 },
];
