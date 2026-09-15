# Pause

Admin-only `pause` and `unpause` toggle instance storage `DataKey::Paused`.

While paused, register, list, offer, accept, reject, cancel, and finalize return `Error::Paused`. Reads (`get_*`, `version`, `admin`, `is_paused`) stay available.
