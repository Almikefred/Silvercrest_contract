# SilverKrest Contract

Soroban property registry for tokenized real estate on Stellar.

On-chain API: register property, create listing, create/accept offer, finalize sale. Companion modules document errors, events, storage keys, and dummy indexer fixtures.

```bash
cargo build --target wasm32-unknown-unknown --release
```

Do not commit deployer secrets. Fixture IDs in `src/dummy_prop_*.rs` are off-chain demo data.
