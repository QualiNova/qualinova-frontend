use crate::types::*;
use soroban_sdk::{Address, Env, String, Vec};

pub fn register_authority(
    env: Env,
    name: String,
    public_key: Address,
    accreditation_info: String,
    allowed_cert_types: Vec<String>,
) -> String {
    public_key.require_auth();

    // Validate inputs
    if name.is_empty() {
        panic!("Name cannot be empty");
    }
    if accreditation_info.is_empty() {
        panic!("Accreditation info cannot be empty");
    }
    // Check for duplicate certification types
    let mut seen = Vec::new(&env);
    for cert_type in allowed_cert_types.iter() {
        if seen.contains(&cert_type) {
            panic!("Certification types must be unique");
        }
        seen.push_back(cert_type);
    }

    // Check if authority already exists
    if env
        .storage()
        .instance()
        .has(&DataKey::Authorities(public_key.clone()))
    {
        panic!("Authority with this public key already exists");
    }

    // Generate unique authority ID
    let authority_id_num: u32 = env
        .storage()
        .instance()
        .get(&DataKey::NextAuthorityId)
        .unwrap_or(1u32);

    // Construct authority_id directly (supporting 1–10 for simplicity)
    let authority_id = if authority_id_num == 0 {
        panic!("Invalid authority ID: 0 not allowed");
    } else if authority_id_num <= 10 {
        String::from_str(
            &env,
            match authority_id_num {
                1 => "AUTH_1",
                2 => "AUTH_2",
                3 => "AUTH_3",
                4 => "AUTH_4",
                5 => "AUTH_5",
                6 => "AUTH_6",
                7 => "AUTH_7",
                8 => "AUTH_8",
                9 => "AUTH_9",
                10 => "AUTH_10",
                _ => panic!(
                    "Authority ID {} not supported in this implementation",
                    authority_id_num
                ),
            },
        )
    } else {
        panic!("Authority ID too large: {}", authority_id_num);
    };

    // Increment authority ID counter
    env.storage()
        .instance()
        .set(&DataKey::NextAuthorityId, &(authority_id_num + 1));

    // Create authority
    let authority = Authority {
        authority_id: authority_id.clone(),
        name,
        public_key: public_key.clone(),
        registration_date: env.ledger().timestamp(),
        accreditation_info,
        allowed_cert_types,
        status: AuthorityStatus::Active,
    };

    // Store authority
    env.storage()
        .instance()
        .set(&DataKey::Authorities(public_key.clone()), &authority);

    // Update authority IDs list
    let mut authority_ids: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::AuthorityIds)
        .unwrap_or_else(|| Vec::new(&env));
    authority_ids.push_back(public_key);
    env.storage()
        .instance()
        .set(&DataKey::AuthorityIds, &authority_ids);

    authority_id
}

pub fn update_authority_info(env: Env, authority_id: Address, field: String, value: String) {
    authority_id.require_auth();

    let mut authority: Authority = env
        .storage()
        .instance()
        .get(&DataKey::Authorities(authority_id.clone()))
        .unwrap_or_else(|| panic!("Authority not found"));

    if authority.status != AuthorityStatus::Active {
        panic!("Cannot update inactive authority");
    }

    // Update the specified field
    if field == String::from_str(&env, "name") {
        authority.name = value;
    } else if field == String::from_str(&env, "accreditation_info") {
        authority.accreditation_info = value;
    } else {
        panic!("Invalid field");
    }

    env.storage()
        .instance()
        .set(&DataKey::Authorities(authority_id), &authority);
}

pub fn verify_authority(env: Env, authority_id: Address) -> bool {
    let authority: Option<Authority> = env
        .storage()
        .instance()
        .get(&DataKey::Authorities(authority_id));
    authority.map_or(false, |auth| auth.status == AuthorityStatus::Active)
}

pub fn deactivate_authority(env: Env, authority_id: Address) {
    authority_id.require_auth();

    let mut authority: Authority = env
        .storage()
        .instance()
        .get(&DataKey::Authorities(authority_id.clone()))
        .unwrap_or_else(|| panic!("Authority not found"));

    if authority.status == AuthorityStatus::Inactive {
        panic!("Authority already inactive");
    }

    authority.status = AuthorityStatus::Inactive;
    env.storage()
        .instance()
        .set(&DataKey::Authorities(authority_id), &authority);
}

pub fn add_certification_type(env: Env, authority_id: Address, cert_type: String) {
    authority_id.require_auth();

    let mut authority: Authority = env
        .storage()
        .instance()
        .get(&DataKey::Authorities(authority_id.clone()))
        .unwrap_or_else(|| panic!("Authority not found"));

    if authority.status != AuthorityStatus::Active {
        panic!("Cannot modify inactive authority");
    }

    if authority.allowed_cert_types.contains(&cert_type) {
        panic!("Certification type already exists");
    }

    authority.allowed_cert_types.push_back(cert_type);
    env.storage()
        .instance()
        .set(&DataKey::Authorities(authority_id), &authority);
}

pub fn remove_certification_type(env: Env, authority_id: Address, cert_type: String) {
    authority_id.require_auth();

    let mut authority: Authority = env
        .storage()
        .instance()
        .get(&DataKey::Authorities(authority_id.clone()))
        .unwrap_or_else(|| panic!("Authority not found"));

    if authority.status != AuthorityStatus::Active {
        panic!("Cannot modify inactive authority");
    }

    let mut new_cert_types: Vec<String> = Vec::new(&env);
    let mut found = false;
    for ct in authority.allowed_cert_types.iter() {
        if ct != cert_type {
            new_cert_types.push_back(ct);
        } else {
            found = true;
        }
    }

    if !found {
        panic!("Certification type not found");
    }

    authority.allowed_cert_types = new_cert_types;
    env.storage()
        .instance()
        .set(&DataKey::Authorities(authority_id), &authority);
}

pub fn get_authority(env: Env, authority_id: Address) -> Authority {
    env.storage()
        .instance()
        .get(&DataKey::Authorities(authority_id))
        .unwrap_or_else(|| panic!("Authority not found"))
}
