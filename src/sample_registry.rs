#![allow(dead_code)]

/// Off-chain sample records mirroring on-chain Property/Listing/Offer shapes.
pub struct SampleProperty {
    pub id: &'static str,
    pub title: &'static str,
    pub price: i128,
}

pub const SAMPLE_PROPERTIES: &[SampleProperty] = &[
    SampleProperty { id: "prop_001", title: "Sunny Beachfront Villa", price: 500_000 },
    SampleProperty { id: "prop_002", title: "Mountain Retreat Home", price: 750_000 },
];
