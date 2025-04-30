use crate::{Certificate, CertificateId};
use alloc::format;
use soroban_sdk::{Address, Bytes, Env, Symbol, Vec};
use soroban_sdk::xdr::ToXdr;

// Storage keys format
const OWNER_CERTS_PREFIX: &str = "owner_certs";

// Events
const CERTIFICATE_TRANSFERRED_EVENT: &str = "certificate_transferred";

// Transfer a certificate to a new owner
pub fn transfer_certificate(env: &Env, certificate_id: CertificateId, new_owner: Address) -> bool {
    // Get the certificate
    let cert_id_bytes = env.crypto().sha256(&Bytes::from_slice(env, &certificate_id.0.to_array()));
    let certificate_key = get_certificate_key(env, &cert_id_bytes);
    
    let mut certificate: Certificate = env
        .storage()
        .persistent()
        .get(&certificate_key)
        .unwrap_or_else(|| panic!("Certificate not found"));
    
    // Check if the certificate is revoked
    if certificate.revoked {
        return false;
    }
    
    // Require authorization from both current and new owner
    certificate.owner.require_auth();
    new_owner.require_auth();
    
    // Update owner lists
    remove_from_owner_certificates(env, &certificate.owner, &certificate_id);
    add_to_owner_certificates(env, &new_owner, &certificate_id);
    
    // Store the old owner for the event
    let old_owner = certificate.owner.clone();
    
    // Update the certificate
    certificate.owner = new_owner.clone();
    env.storage().persistent().set(&certificate_key, &certificate);
    
    // Emit certificate transferred event
    env.events().publish(
        (Symbol::new(env, CERTIFICATE_TRANSFERRED_EVENT),),
        (certificate_id, old_owner, new_owner),
    );
    
    true
}

// Remove a certificate from the owner's list
fn remove_from_owner_certificates(env: &Env, owner: &Address, cert_id: &CertificateId) {
    let owner_key = get_owner_key(env, owner);
    
    let owner_certs: Vec<CertificateId> = env
        .storage()
        .persistent()
        .get(&owner_key)
        .unwrap_or_else(|| Vec::new(env));
    
    // Create a new list without the removed certificate
    let mut new_owner_certs = Vec::new(env);
    
    for i in 0..owner_certs.len() {
        let current = owner_certs.get(i).unwrap();
        if current.0 != cert_id.0 {
            new_owner_certs.push_back(current);
        }
    }
    
    // Update the owner's certificates list
    env.storage().persistent().set(&owner_key, &new_owner_certs);
}

// Add a certificate to the owner's list
fn add_to_owner_certificates(env: &Env, owner: &Address, cert_id: &CertificateId) {
    let owner_key = get_owner_key(env, owner);
    
    let mut owner_certs: Vec<CertificateId> = env
        .storage()
        .persistent()
        .get(&owner_key)
        .unwrap_or_else(|| Vec::new(env));
    
    owner_certs.push_back(cert_id.clone());
    env.storage().persistent().set(&owner_key, &owner_certs);
}

// Helper function to get a certificate key from a hash
fn get_certificate_key(env: &Env, hash: &soroban_sdk::crypto::Hash<32>) -> Symbol {
    // Create a simple key with the hash - avoid using colons
    let key = format!("cert{:x}", hash.to_array()[0]);
    Symbol::new(env, &key)
}

// Helper function to get an owner key
fn get_owner_key(env: &Env, owner: &Address) -> Symbol {
    let owner_hash = env.crypto().sha256(&owner.clone().to_xdr(env));
    // Avoid using colons in Symbol keys
    let key = format!("{}_{:x}", OWNER_CERTS_PREFIX, owner_hash.to_array()[0]);
    Symbol::new(env, &key)
}
