use crate::types::*;
use soroban_sdk::{Address, Env, String, Vec};

pub fn register_entity(
    env: Env,
    name: String,
    industry_sector: String,
    location: String,
    contact_info: String,
    public_key: Address,
) -> String {
    public_key.require_auth();

    // Validate inputs
    if name.is_empty() {
        panic!("Name cannot be empty");
    }
    if industry_sector.is_empty() {
        panic!("Industry sector cannot be empty");
    }
    if location.is_empty() {
        panic!("Location cannot be empty");
    }
    if contact_info.is_empty() {
        panic!("Contact info cannot be empty");
    }

    // Check if entity already exists
    if env
        .storage()
        .instance()
        .has(&DataKey::Entities(public_key.clone()))
    {
        panic!("Entity with this public key already exists");
    }

    // Generate unique entity ID
    let entity_id_num: u32 = env
        .storage()
        .instance()
        .get(&DataKey::NextEntityId)
        .unwrap_or(1u32);

    // Construct entity_id
    let entity_id = if entity_id_num == 0 {
        panic!("Invalid entity ID: 0 not allowed");
    } else {
        // Using numbering scheme ENT_1, ENT_2, etc.
        match entity_id_num {
            1 => String::from_str(&env, "ENT_1"),
            2 => String::from_str(&env, "ENT_2"),
            3 => String::from_str(&env, "ENT_3"),
            4 => String::from_str(&env, "ENT_4"),
            5 => String::from_str(&env, "ENT_5"),
            6 => String::from_str(&env, "ENT_6"),
            7 => String::from_str(&env, "ENT_7"),
            8 => String::from_str(&env, "ENT_8"),
            9 => String::from_str(&env, "ENT_9"),
            10 => String::from_str(&env, "ENT_10"),
            _ => panic!("Entity ID too large: {}", entity_id_num),
        }
    };

    // Increment entity ID counter
    env.storage()
        .instance()
        .set(&DataKey::NextEntityId, &(entity_id_num + 1));

    // Create entity
    let entity = Entity {
        entity_id: entity_id.clone(),
        name,
        public_key: public_key.clone(),
        registration_date: env.ledger().timestamp(),
        industry_sector,
        location,
        contact_info,
        status: EntityStatus::Active,
        certifications: Vec::new(&env),
    };

    // Store entity
    env.storage()
        .instance()
        .set(&DataKey::Entities(public_key.clone()), &entity);

    // Update entity IDs list
    let mut entity_ids: Vec<Address> = env
        .storage()
        .instance()
        .get(&DataKey::EntityIds)
        .unwrap_or_else(|| Vec::new(&env));
    entity_ids.push_back(public_key);
    env.storage()
        .instance()
        .set(&DataKey::EntityIds, &entity_ids);

    entity_id
}

pub fn update_entity_info(env: Env, entity_id: Address, field: String, value: String) {
    entity_id.require_auth();

    let mut entity: Entity = env
        .storage()
        .instance()
        .get(&DataKey::Entities(entity_id.clone()))
        .unwrap_or_else(|| panic!("Entity not found"));

    if entity.status != EntityStatus::Active {
        panic!("Cannot update inactive entity");
    }

    // Update the specified field
    if field == String::from_str(&env, "name") {
        if value.is_empty() {
            panic!("Name cannot be empty");
        }
        entity.name = value;
    } else if field == String::from_str(&env, "industry_sector") {
        if value.is_empty() {
            panic!("Industry sector cannot be empty");
        }
        entity.industry_sector = value;
    } else if field == String::from_str(&env, "location") {
        if value.is_empty() {
            panic!("Location cannot be empty");
        }
        entity.location = value;
    } else if field == String::from_str(&env, "contact_info") {
        if value.is_empty() {
            panic!("Contact info cannot be empty");
        }
        entity.contact_info = value;
    } else {
        panic!("Invalid field");
    }

    env.storage()
        .instance()
        .set(&DataKey::Entities(entity_id), &entity);
}

pub fn verify_entity(env: Env, entity_id: Address) -> bool {
    let entity: Option<Entity> = env
        .storage()
        .instance()
        .get(&DataKey::Entities(entity_id));

    entity.map_or(false, |ent| ent.status == EntityStatus::Active)
}

pub fn deactivate_entity(env: Env, entity_id: Address, _reason: String) {
    entity_id.require_auth();

    let mut entity: Entity = env
        .storage()
        .instance()
        .get(&DataKey::Entities(entity_id.clone()))
        .unwrap_or_else(|| panic!("Entity not found"));

    if entity.status == EntityStatus::Inactive {
        panic!("Entity already inactive");
    }

    // Deactivate the entity
    entity.status = EntityStatus::Inactive;
    env.storage()
        .instance()
        .set(&DataKey::Entities(entity_id), &entity);
}

pub fn add_certification(env: Env, entity_id: Address, certification_id: String) {
    let mut entity: Entity = env
        .storage()
        .instance()
        .get(&DataKey::Entities(entity_id.clone()))
        .unwrap_or_else(|| panic!("Entity not found"));

    if entity.status != EntityStatus::Active {
        panic!("Cannot add certification to inactive entity");
    }

    if entity.certifications.contains(&certification_id) {
        panic!("Certification already associated with this entity");
    }

    entity.certifications.push_back(certification_id);
    env.storage()
        .instance()
        .set(&DataKey::Entities(entity_id), &entity);
}

pub fn list_entity_certifications(env: Env, entity_id: Address) -> Vec<String> {
    let entity: Entity = env
        .storage()
        .instance()
        .get(&DataKey::Entities(entity_id))
        .unwrap_or_else(|| panic!("Entity not found"));

    entity.certifications
}

pub fn get_entity(env: Env, entity_id: Address) -> Entity {
    env.storage()
        .instance()
        .get(&DataKey::Entities(entity_id))
        .unwrap_or_else(|| panic!("Entity not found"))
}