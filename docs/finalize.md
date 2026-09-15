# Finalize

Requires an accepted offer that matches the listing id, listing status pending, and buyer `require_auth`.

Marks the listing sold and writes `property.owner = buyer`. NFT custody transfer remains a coordinated off-chain step against `nft_contract` / `nft_id`.
