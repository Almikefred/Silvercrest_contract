#![allow(dead_code)]

/// Sample share lot used by off-chain indexer tests.
pub struct ShareLot {
    pub property_id: &'static str,
    pub shares: i128,
    pub share_price: i128,
}

pub fn cost(lot: &ShareLot) -> i128 {
    lot.shares.saturating_mul(lot.share_price)
}
