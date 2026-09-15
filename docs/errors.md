# Errors

`src/errors.rs` defines `#[contracterror]` codes used by the registry:

| Code | Name | When |
| --- | --- | --- |
| 1 | NotFound | Missing property, listing, offer, or admin |
| 2 | Unauthorized | Caller is not the property owner / seller |
| 3 | InvalidState | Offer/listing status does not allow the call |
| 4 | AlreadyExists | Duplicate property, listing, or offer id |
| 6 | ZeroPrice | `price <= 0` |
| 7 | Paused | Admin pause is on |
| 9 | AlreadyInitialized | `initialize` called twice |
| 10 | ListingNotActive | Offer against a pending/sold listing |
| 11 | OfferNotPending | Accept/reject on a settled offer |
