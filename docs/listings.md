# Listings

status 0 active, 1 pending (offer accepted), 2 sold or cancelled.

`create_listing` requires the seller to be `property.owner`. `accept_offer` moves an active listing to pending. `finalize_sale` marks it sold. `cancel_listing` is only valid while active.
