#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, Vec};

mod types;
mod error;
mod storage;
#[cfg(test)]
mod test;

pub use types::{CertificationType, CertTypeStatus};
pub use error::Error;
use storage::Storage;

#[contract]
pub struct CertificationTypeManagementContract;

// Define a trait for the contract interface
pub trait CertificationTypeManagementTrait {
    fn register_certification_type(
        env: Env,
        name: String,
        version: String,
        description: String,
        industry_scope: Vec<String>,
        validity_period: String,
    ) -> Result<String, Error>;

    fn update_certification_type(
        env: Env,
        cert_type_id: String,
        field: String,
        value: String,
    ) -> Result<(), Error>;

    fn deprecate_certification_type(
        env: Env,
        cert_type_id: String,
        reason: String,
    ) -> Result<(), Error>;

    fn list_all_certification_types(env: Env) -> Vec<CertificationType>;

    fn get_certification_type_details(
        env: Env,
        cert_type_id: String,
    ) -> Result<CertificationType, Error>;

    fn set_required_evidence(
        env: Env,
        cert_type_id: String,
        evidence_list: Vec<String>,
    ) -> Result<(), Error>;

    fn assign_authority_to_cert_type(
        env: Env,
        cert_type_id: String,
        authority_id: String,
    ) -> Result<(), Error>;
}

#[contractimpl]
impl CertificationTypeManagementTrait for CertificationTypeManagementContract {
    // Registers a new certification type
    fn register_certification_type(
        env: Env,
        name: String,
        version: String,
        description: String,
        industry_scope: Vec<String>,
        validity_period: String,
    ) -> Result<String, Error> {
        let storage = Storage::new(&env);
        storage.register_certification_type(name, version, description, industry_scope, validity_period)
    }

    // Updates a specific field of a certification type
    fn update_certification_type(
        env: Env,
        cert_type_id: String,
        field: String,
        value: String,
    ) -> Result<(), Error> {
        let storage = Storage::new(&env);
        storage.update_certification_type(cert_type_id, field, value)
    }

    // Deprecates a certification type
    fn deprecate_certification_type(
        env: Env,
        cert_type_id: String,
        reason: String,
    ) -> Result<(), Error> {
        let storage = Storage::new(&env);
        storage.deprecate_certification_type(cert_type_id, reason)
    }

    // Lists all certification types
    fn list_all_certification_types(env: Env) -> Vec<CertificationType> {
        let storage = Storage::new(&env);
        storage.list_all_certification_types()
    }

    // Gets details of a specific certification type
    fn get_certification_type_details(
        env: Env,
        cert_type_id: String,
    ) -> Result<CertificationType, Error> {
        let storage = Storage::new(&env);
        storage.get_certification_type_details(cert_type_id)
    }

    // Sets required evidence for a certification type
    fn set_required_evidence(
        env: Env,
        cert_type_id: String,
        evidence_list: Vec<String>,
    ) -> Result<(), Error> {
        let storage = Storage::new(&env);
        storage.set_required_evidence(cert_type_id, evidence_list)
    }

    // Assigns an authority to a certification type
    fn assign_authority_to_cert_type(
        env: Env,
        cert_type_id: String,
        authority_id: String,
    ) -> Result<(), Error> {
        let storage = Storage::new(&env);
        storage.assign_authority_to_cert_type(cert_type_id, authority_id)
    }
}