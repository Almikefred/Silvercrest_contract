/// Off-chain share-lot helper used by indexer notes and unit tests.

pub struct ShareLot {
    pub property_id: &'static str,
    pub shares: i128,
    pub share_price: i128,
}

pub fn cost(lot: &ShareLot) -> i128 {
    lot.shares.saturating_mul(lot.share_price)
}

#[cfg(test)]
mod tests {
    use super::{cost, ShareLot};

    #[test]
    fn cost_multiplies_shares() {
        let lot = ShareLot {
            property_id: "prop_001",
            shares: 10,
            share_price: 250,
        };
        assert_eq!(cost(&lot), 2_500);
    }
}
