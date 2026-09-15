# Storage

Instance: admin, version, paused flag.

Persistent maps use `DataKey::{Property, Listing, Offer}(id)` so the three id spaces cannot collide. TTL is extended on write.
