# Authorization

- Admin: `initialize`, `pause`, `unpause`
- Owner/seller: `register_property`, `create_listing`, `cancel_listing`, `accept_offer`, `reject_offer`
- Buyer: `create_offer`, `finalize_sale`

All of the above call `Address::require_auth` on the acting party.
