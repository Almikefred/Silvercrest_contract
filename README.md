# SilverKrest Contract

Soroban property registry for tokenized real estate on Stellar.

On-chain API (v0.3): admin `initialize` / `pause` / `unpause`, register property, create or cancel listing, create / accept / reject offer, finalize sale with owner transfer. Typed errors, events, and positive-price checks are enforced on every mutation.

```bash
cargo test
cargo build --target wasm32-unknown-unknown --release
```

Do not commit deployer secrets. Fixture IDs in `src/sample_prop_*.rs` are off-chain catalog samples for indexer walkthroughs.
