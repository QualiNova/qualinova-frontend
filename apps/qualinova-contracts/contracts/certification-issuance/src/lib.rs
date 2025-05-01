// Disable the standard library for Soroban compatibility
#![no_std]

// Import external crates
extern crate alloc;

// Add global allocator
#[cfg(target_arch = "wasm32")]
mod wasm_allocator {
    use wee_alloc;
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
}

// Import modules
mod access_control;
mod issuance;
mod query;
mod transfer;
mod verification;
#[cfg(test)]
mod test;

// Import from the Soroban SDK
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, Map, String, Vec,
};

// Define the certificate ID type
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateId(pub BytesN<32>);

// Define the certificate metadata structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateMetadata {
    pub title: String,
    pub description: String,
    pub achievement_type: String,
    pub additional_data: Map<String, BytesN<32>>,
}

// Define the certificate structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate {
    pub id: CertificateId,
    pub owner: Address,
    pub issuer: Address,
    pub metadata: CertificateMetadata,
    pub issuance_date: u64,
    pub expiration_date: Option<u64>,
    pub revoked: bool,
    pub signature: Bytes,
}

// Define the contract
#[contract]
pub struct CertificationContract;

// Implement the contract
#[contractimpl]
impl CertificationContract {
    // === Initialization ===
    
    // Initialize the contract with an admin
    pub fn initialize(env: &Env, admin: Address) -> bool {
        access_control::initialize(env, &admin);
        true
    }
    
    // === Access Control ===
    
    // Get the admin address
    pub fn get_admin(env: &Env) -> Address {
        access_control::get_admin(env)
    }
    
    // Transfer admin role to a new address
    pub fn transfer_admin(env: &Env, new_admin: Address) -> bool {
        access_control::transfer_admin(env, new_admin)
    }
    
    // Add an issuer
    pub fn add_issuer(env: &Env, issuer: Address) -> bool {
        access_control::add_issuer(env, issuer)
    }
    
    // Remove an issuer
    pub fn remove_issuer(env: &Env, issuer: Address) -> bool {
        access_control::remove_issuer(env, issuer)
    }
    
    // Check if an address is an issuer
    pub fn is_issuer(env: &Env, address: Address) -> bool {
        access_control::is_issuer(env, address)
    }
    
    // Get all issuers
    pub fn get_issuers(env: &Env) -> Vec<Address> {
        access_control::get_issuers(env)
    }
    
    // === Certificate Issuance ===
    
    // Issue a new certificate
    pub fn issue_certificate(
        env: &Env,
        owner: Address,
        metadata: CertificateMetadata,
        expiration_date: Option<u64>,
        signature: Bytes,
    ) -> CertificateId {
        issuance::issue_certificate(env, owner, metadata, expiration_date, signature)
    }
    
    // Batch issue multiple certificates
    pub fn batch_issue_certificates(
        env: &Env,
        owners: Vec<Address>,
        metadatas: Vec<CertificateMetadata>,
        expiration_dates: Vec<Option<u64>>,
        signatures: Vec<Bytes>,
    ) -> Vec<CertificateId> {
        issuance::batch_issue_certificates(env, owners, metadatas, expiration_dates, signatures)
    }
    
    // Get the total certificate count
    pub fn get_certificate_count(env: &Env) -> u32 {
        issuance::get_certificate_count(env)
    }
    
    // === Certificate Queries ===
    
    // Get a certificate by its ID
    pub fn get_certificate(env: &Env, certificate_id: CertificateId) -> Certificate {
        query::get_certificate(env, certificate_id)
    }
    
    // List certificates by owner with pagination
    pub fn list_certificates_by_owner(
        env: &Env,
        owner: Address,
        start_index: u32,
        limit: u32,
    ) -> Vec<Certificate> {
        query::list_certificates_by_owner(env, owner, start_index, limit)
    }
    
    // List certificates by issuer with pagination
    pub fn list_certificates_by_issuer(
        env: &Env,
        issuer: Address,
        start_index: u32,
        limit: u32,
    ) -> Vec<Certificate> {
        query::list_certificates_by_issuer(env, issuer, start_index, limit)
    }
    
    // Count certificates by owner
    pub fn count_certificates_by_owner(env: &Env, owner: Address) -> u32 {
        query::count_certificates_by_owner(env, &owner)
    }
    
    // Count certificates by issuer
    pub fn count_certificates_by_issuer(env: &Env, issuer: Address) -> u32 {
        query::count_certificates_by_issuer(env, &issuer)
    }
    
    // === Certificate Transfer ===
    
    // Transfer a certificate to a new owner
    pub fn transfer_certificate(
        env: &Env,
        certificate_id: CertificateId,
        new_owner: Address,
    ) -> bool {
        transfer::transfer_certificate(env, certificate_id, new_owner)
    }
    
    // === Certificate Verification ===
    
    // Verify a certificate's signature
    pub fn verify_certificate_signature(env: &Env, certificate_id: CertificateId) -> bool {
        verification::verify_certificate_signature(env, certificate_id)
    }
    
    // === Certificate Revocation ===
    
    // Revoke a certificate
    pub fn revoke_certificate(env: &Env, certificate_id: CertificateId) -> bool {
        access_control::revoke_certificate(env, certificate_id)
    }
}
