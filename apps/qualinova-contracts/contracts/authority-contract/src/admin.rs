use crate::types::*;
use soroban_sdk::{Address, Env, Vec};

pub fn initialize(env: Env, admin: Address) {
    admin.require_auth();

    // Initialize authority IDs list
    let authority_ids: Vec<Address> = Vec::new(&env);
    env.storage()
        .instance()
        .set(&DataKey::AuthorityIds, &authority_ids);

    // Initialize authority counter
    env.storage()
        .instance()
        .set(&DataKey::NextAuthorityId, &1u32);
}
