# Testing

```bash
cargo test
cargo build --lib
```

`cargo test` covers share-lot math, crate version `3`, and `Error` discriminants (`ZeroPrice`, `Paused`, `AlreadyInitialized`).

Host-backed sale path (Stellar CLI / `soroban-sdk` testutils):

1. `initialize(admin)` stores version 3 and `paused=false`
2. `pause` blocks `register_property`; `unpause` restores it
3. `price <= 0` returns `Error::ZeroPrice`
4. `create_listing` requires `seller == property.owner`
5. `accept_offer` sets listing status to pending
6. `finalize_sale` transfers `property.owner` to the buyer and marks the listing sold
7. `reject_offer` leaves the listing active
