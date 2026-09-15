use soroban_sdk::{symbol_short, Address, Env, String};

pub fn register(env: &Env, owner: &Address, id: &String, price: i128) {
    env.events()
        .publish((symbol_short!("register"), owner.clone()), (id.clone(), price));
}

pub fn list(env: &Env, seller: &Address, id: &String, price: i128) {
    env.events()
        .publish((symbol_short!("list"), seller.clone()), (id.clone(), price));
}

pub fn offer(env: &Env, buyer: &Address, id: &String, price: i128) {
    env.events()
        .publish((symbol_short!("offer"), buyer.clone()), (id.clone(), price));
}

pub fn accept(env: &Env, seller: &Address, offer_id: &String) {
    env.events()
        .publish((symbol_short!("accept"), seller.clone()), offer_id.clone());
}

pub fn reject(env: &Env, seller: &Address, offer_id: &String) {
    env.events()
        .publish((symbol_short!("reject"), seller.clone()), offer_id.clone());
}

pub fn finalize(env: &Env, buyer: &Address, listing_id: &String) {
    env.events()
        .publish((symbol_short!("finalize"), buyer.clone()), listing_id.clone());
}

pub fn pause(env: &Env, admin: &Address, paused: bool) {
    env.events()
        .publish((symbol_short!("pause"), admin.clone()), paused);
}

pub fn cancel(env: &Env, seller: &Address, listing_id: &String) {
    env.events()
        .publish((symbol_short!("cancel"), seller.clone()), listing_id.clone());
}
