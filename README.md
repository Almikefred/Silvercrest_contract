# SilverKrest Contract

Soroban smart contracts for tokenized real estate on Stellar.

## Overview

The property registry contract enables:
- **Property Registration** — onboard tokenized real estate with NFT backing
- **Listings** — create buyable/sellable listings
- **Offers** — submit and accept purchase offers
- **Sale Finalization** — atomic transfer of ownership

## Building

```bash
cargo build --target wasm32-unknown-unknown --release
```

Compiled WASM will be in `target/wasm32-unknown-unknown/release/silvercrest_contract.wasm`.

## Deploying to Testnet

```bash
soroban contract deploy --wasm target/wasm32-unknown-unknown/release/silvercrest_contract.wasm --network testnet
```

## Contract API

### initialize(env)
Set contract version. Called once at deployment.

### register_property(env, id, title, location, price, currency, owner, nft_contract, nft_id, metadata_uri)
Register a new property, backed by an NFT. Requires owner's signature.

**Returns:** `Property` struct with creation timestamp.

### get_property(env, id)
Fetch a registered property by ID.

### create_listing(env, id, property_id, seller, price, currency)
Create a buyable listing for a property. Requires seller's signature.

**Returns:** `Listing` with status=0 (active).

### create_offer(env, id, listing_id, buyer, price)
Submit an offer to purchase. Requires buyer's signature.

**Returns:** `Offer` with status=0 (pending).

### accept_offer(env, offer_id)
Accept a pending offer. Requires listing seller's signature.

**Returns:** Updated `Offer` with status=1 (accepted).

### finalize_sale(env, offer_id, listing_id)
Complete the sale (transfer NFT). Requires buyer's signature.

Updates listing status to 2 (sold).

## Data Structures

**Property**
- `id`: unique property identifier
- `title`, `location`: metadata
- `price`: in minor units (i128)
- `currency`: "USD", "XLM", etc.
- `owner`: Stellar address
- `nft_contract`, `nft_id`: NFT reference
- `metadata_uri`: IPFS or HTTP link to full metadata
- `created_at`: ledger timestamp

**Listing**
- `id`: unique listing identifier
- `property_id`: links to a property
- `seller`: Stellar address
- `price`, `currency`: asking price
- `status`: 0=active, 1=pending, 2=sold
- `created_at`: ledger timestamp

**Offer**
- `id`: unique offer identifier
- `listing_id`: which listing is being offered on
- `buyer`: Stellar address
- `price`: offered price
- `status`: 0=pending, 1=accepted, 2=rejected
- `created_at`: ledger timestamp

## Security Notes

- All state-changing functions require authorization (require_auth)
- No custody of funds — contract coordinates transfers only
- NFT ownership verified by caller's signature + contract reference
- Offer acceptance & finalization are separate to allow escrow flows