#![no_std]
use soroban_sdk::{Address, Env, String, Vec, contract, contractimpl};

mod admin;
mod entity;
mod test;
mod types;

pub use admin::*;
pub use entity::*;
pub use types::*;

#[contract]
pub struct CertifiableEntity;

#[contractimpl]
impl CertifiableEntity {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        admin::initialize(env, admin);
    }

    // Entity management functions
    pub fn register_entity(
        env: Env,
        name: String,
        industry_sector: String,
        location: String,
        contact_info: String,
        public_key: Address,
    ) -> String {
        entity::register_entity(
            env,
            name,
            industry_sector,
            location,
            contact_info,
            public_key,
        )
    }

    pub fn update_entity_info(env: Env, entity_id: Address, field: String, value: String) {
        entity::update_entity_info(env, entity_id, field, value)
    }

    pub fn verify_entity(env: Env, entity_id: Address) -> bool {
        entity::verify_entity(env, entity_id)
    }

    pub fn deactivate_entity(env: Env, entity_id: Address, reason: String) {
        entity::deactivate_entity(env, entity_id, reason)
    }

    pub fn list_entity_certifications(env: Env, entity_id: Address) -> Vec<String> {
        entity::list_entity_certifications(env, entity_id)
    }

    pub fn get_entity(env: Env, entity_id: Address) -> Entity {
        entity::get_entity(env, entity_id)
    }

    // Admin functions
    pub fn get_admin(env: Env) -> Address {
        admin::get_admin(env)
    }

    pub fn transfer_admin(env: Env, current_admin: Address, new_admin: Address) {
        admin::transfer_admin(env, current_admin, new_admin)
    }
}