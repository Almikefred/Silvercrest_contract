# Contract overview

Soroban property registry for tokenized real estate on Stellar.

The on-chain crate (`src/lib.rs`) stores properties, listings, and offers behind typed storage keys. Admin pause, events, and `Error` codes live in companion modules. Off-chain catalog fixtures live under `src/sample_prop_*.rs` for indexer walkthroughs.

## Current surface (0.3.0)

- `initialize(admin)` writes version `3`, admin, and pause=false
- `pause` / `unpause` / `is_paused` (admin auth)
- `register_property` / `get_property` (rejects `price <= 0` and duplicate ids)
- `create_listing` / `get_listing` / `cancel_listing` (seller must own the property)
- `create_offer` / `get_offer` / `accept_offer` / `reject_offer`
- `finalize_sale` marks the listing sold and transfers `property.owner` to the buyer
