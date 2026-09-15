# Contract overview

Soroban property registry for tokenized real estate on Stellar.

The on-chain crate (`src/lib.rs`) stores properties, listings, and offers. Companion modules cover errors, events, fractional share lots, and storage notes. Off-chain catalog fixtures live under `src/sample_prop_*.rs` for indexer walkthroughs.

## Current surface

- `initialize` writes instance storage version `2` (aligned with crate 0.2.0)
- `register_property` / `get_property`
- `create_listing` / `get_listing`
- `create_offer` / `get_offer` / `accept_offer`
- `finalize_sale` marks a listing sold after buyer auth
