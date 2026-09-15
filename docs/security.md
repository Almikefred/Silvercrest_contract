# Security

- Never commit deployer secrets.
- `initialize` binds a single admin; pause/unpause require that address.
- Property, listing, offer, and finalize paths call `Address::require_auth` on the acting party.
- Duplicate ids, non-positive prices, and sold listings are rejected on-chain.
- `finalize_sale` requires an accepted offer, a pending listing, and buyer auth before transferring `property.owner`.
