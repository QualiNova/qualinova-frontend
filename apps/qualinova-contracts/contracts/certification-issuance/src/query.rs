use crate::{issuance, Certificate, CertificateId};
use alloc::format;
use core::cmp;
use soroban_sdk::{Address, Bytes, Env, Symbol, Vec};
use soroban_sdk::xdr::ToXdr;

// Storage keys format
const ISSUER_CERTS_PREFIX: &str = "issuer_certs";

// Get a certificate by its ID
pub fn get_certificate(env: &Env, certificate_id: CertificateId) -> Certificate {
    let cert_id_bytes = env.crypto().sha256(&Bytes::from_slice(env, &certificate_id.0.to_array()));
    let certificate_key = get_certificate_key(env, &cert_id_bytes);
    
    env.storage()
        .persistent()
        .get(&certificate_key)
        .unwrap_or_else(|| panic!("Certificate not found"))
}

// List certificates by owner with pagination
pub fn list_certificates_by_owner(
    env: &Env,
    owner: Address,
    start_index: u32,
    limit: u32,
) -> Vec<Certificate> {
    // Get all certificate IDs for the owner
    let cert_ids = issuance::get_owner_certificates(env, &owner);
    
    // Apply pagination
    let start = if start_index >= cert_ids.len() {
        return Vec::new(env);
    } else {
        start_index as u32
    };
    
    let end = cmp::min(start + limit, cert_ids.len());
    
    // Fetch certificates
    let mut certificates = Vec::new(env);
    for i in start..end {
        let cert_id = cert_ids.get(i).unwrap();
        let certificate = get_certificate(env, cert_id);
        certificates.push_back(certificate);
    }
    
    certificates
}

// List certificates by issuer with pagination
pub fn list_certificates_by_issuer(
    env: &Env,
    issuer: Address,
    start_index: u32,
    limit: u32,
) -> Vec<Certificate> {
    // Get all certificate IDs for the issuer
    let cert_ids = get_issuer_certificates(env, &issuer);
    
    // Apply pagination
    let start = if start_index >= cert_ids.len() {
        return Vec::new(env);
    } else {
        start_index as u32
    };
    
    let end = cmp::min(start + limit, cert_ids.len());
    
    // Fetch certificates
    let mut certificates = Vec::new(env);
    for i in start..end {
        let cert_id = cert_ids.get(i).unwrap();
        let certificate = get_certificate(env, cert_id);
        certificates.push_back(certificate);
    }
    
    certificates
}

// Get all certificates for an issuer
fn get_issuer_certificates(env: &Env, issuer: &Address) -> Vec<CertificateId> {
    let issuer_key = get_issuer_key(env, issuer);
    
    env.storage()
        .persistent()
        .get(&issuer_key)
        .unwrap_or_else(|| Vec::new(env))
}

// Count certificates by owner
pub fn count_certificates_by_owner(env: &Env, owner: &Address) -> u32 {
    issuance::get_owner_certificates(env, owner).len()
}

// Count certificates by issuer
pub fn count_certificates_by_issuer(env: &Env, issuer: &Address) -> u32 {
    get_issuer_certificates(env, issuer).len()
}

// Helper function to get a certificate key from a hash
fn get_certificate_key(env: &Env, hash: &soroban_sdk::crypto::Hash<32>) -> Symbol {
    // Create a simple key with the hash - avoid using colons
    let key = format!("cert{:x}", hash.to_array()[0]);
    Symbol::new(env, &key)
}

// Helper function to get an issuer key
fn get_issuer_key(env: &Env, issuer: &Address) -> Symbol {
    let issuer_hash = env.crypto().sha256(&issuer.clone().to_xdr(env));
    // Avoid using colons in Symbol keys
    let key = format!("{}_{:x}", ISSUER_CERTS_PREFIX, issuer_hash.to_array()[0]);
    Symbol::new(env, &key)
}
