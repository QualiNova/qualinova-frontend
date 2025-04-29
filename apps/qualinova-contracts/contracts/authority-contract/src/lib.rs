#![no_std]
use soroban_sdk::{Address, Env, String, Vec, contract, contractimpl};

mod admin;
mod authority;
mod test;
mod types;

pub use admin::*;
pub use authority::*;
pub use types::*;

#[contract]
pub struct CertifyingAuthority;

#[contractimpl]
impl CertifyingAuthority {
    // Initialize the contract
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }
    // Authority management functions
    pub fn register_authority(
        env: Env,
        name: String,
        public_key: Address,
        accreditation_info: String,
        allowed_cert_types: Vec<String>,
    ) -> String {
        authority::register_authority(
            env,
            name,
            public_key,
            accreditation_info,
            allowed_cert_types,
        )
    }

    pub fn update_authority_info(env: Env, authority_id: Address, field: String, value: String) {
        authority::update_authority_info(env, authority_id, field, value)
    }

    pub fn verify_authority(env: Env, authority_id: Address) -> bool {
        authority::verify_authority(env, authority_id)
    }

    pub fn deactivate_authority(env: Env, authority_id: Address) {
        authority::deactivate_authority(env, authority_id)
    }

    pub fn add_certification_type(env: Env, authority_id: Address, cert_type: String) {
        authority::add_certification_type(env, authority_id, cert_type)
    }

    pub fn remove_certification_type(env: Env, authority_id: Address, cert_type: String) {
        authority::remove_certification_type(env, authority_id, cert_type)
    }

    pub fn get_authority(env: Env, authority_id: Address) -> Authority {
        authority::get_authority(env, authority_id)
    }
}
