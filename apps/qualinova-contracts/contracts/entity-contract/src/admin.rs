use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

pub fn initialize(env: Env, admin: Address) {
    admin.require_auth();

    // Initialize entity IDs list
    let entity_ids: Vec<Address> = Vec::new(&env);
    env.storage()
        .instance()
        .set(&DataKey::EntityIds, &entity_ids);

    // Initialize entity counter
    env.storage()
        .instance()
        .set(&DataKey::NextEntityId, &1u32);

    // Set admin
    env.storage()
        .instance()
        .set(&DataKey::Admin, &admin);
}

pub fn get_admin(env: Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("Admin not set"))
}

pub fn is_admin(env: Env, address: Address) -> bool {
    let admin = get_admin(env);
    admin == address
}

pub fn require_admin(env: Env, address: Address) {
    address.require_auth();
    if !is_admin(env.clone(), address) {
        panic!("Caller is not the admin");
    }
}

pub fn transfer_admin(env: Env, current_admin: Address, new_admin: Address) {
    require_admin(env.clone(), current_admin);
    env.storage().instance().set(&DataKey::Admin, &new_admin);
}