# Events

Mutations publish topics from `src/events.rs`:

- `register` — owner, property id, price
- `list` — seller, listing id, price
- `offer` — buyer, offer id, price
- `accept` / `reject` — seller, offer id
- `finalize` — buyer, listing id
- `pause` — admin, paused flag
- `cancel` — seller, listing id
