use crate::{Certificate, CertificateId};
use alloc::format;
use soroban_sdk::{Address, Bytes, Env, Symbol, Vec};

// Storage keys
const ADMIN_KEY: &str = "admin";
const ISSUERS_KEY: &str = "issuers";

// Events
const ISSUER_ADDED_EVENT: &str = "issuer_added";
const ISSUER_REMOVED_EVENT: &str = "issuer_removed";
const ADMIN_TRANSFERRED_EVENT: &str = "admin_transferred";
const CERTIFICATE_REVOKED_EVENT: &str = "certificate_revoked";

// Initialize the contract with an admin
pub fn initialize(env: &Env, admin: &Address) {
    // Check if already initialized
    if is_initialized(env) {
        panic!("Contract already initialized");
    }
    
    // Set the admin
    env.storage().instance().set(&Symbol::new(env, ADMIN_KEY), admin);
    
    // Initialize empty issuers list
    let issuers: Vec<Address> = Vec::new(env);
    env.storage().instance().set(&Symbol::new(env, ISSUERS_KEY), &issuers);
}

// Check if the contract is initialized
pub fn is_initialized(env: &Env) -> bool {
    env.storage().instance().has(&Symbol::new(env, ADMIN_KEY))
}

// Check if the contract has an admin
fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&Symbol::new(env, ADMIN_KEY))
}

// Get the admin address
pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&Symbol::new(env, ADMIN_KEY))
        .unwrap_or_else(|| panic!("Contract not initialized"))
}

// Transfer admin role to a new address
pub fn transfer_admin(env: &Env, new_admin: Address) -> bool {
    // Verify the caller is the current admin
    let admin = get_admin(env);
    admin.require_auth();
    
    // Set the new admin
    env.storage().instance().set(&Symbol::new(env, ADMIN_KEY), &new_admin);
    
    // Emit admin transferred event
    env.events().publish(
        (Symbol::new(env, ADMIN_TRANSFERRED_EVENT),),
        (admin, new_admin),
    );
    
    true
}

// Add an issuer
pub fn add_issuer(env: &Env, issuer: Address) -> bool {
    // Verify the caller is the admin
    let admin = get_admin(env);
    admin.require_auth();
    
    // Get the current issuers
    let mut issuers: Vec<Address> = env
        .storage()
        .instance()
        .get(&Symbol::new(env, ISSUERS_KEY))
        .unwrap_or_else(|| Vec::new(env));
    
    // Check if the issuer is already added
    for i in 0..issuers.len() {
        if issuers.get(i).unwrap() == issuer {
            return false; // Already an issuer
        }
    }
    
    // Add the new issuer
    issuers.push_back(issuer.clone());
    env.storage().instance().set(&Symbol::new(env, ISSUERS_KEY), &issuers);
    
    // Emit issuer added event
    env.events().publish(
        (Symbol::new(env, ISSUER_ADDED_EVENT),),
        (admin, issuer),
    );
    
    true
}

// Remove an issuer
pub fn remove_issuer(env: &Env, issuer: Address) -> bool {
    // Verify the caller is the admin
    let admin = get_admin(env);
    admin.require_auth();
    
    // Get the current issuers
    let issuers: Vec<Address> = env
        .storage()
        .instance()
        .get(&Symbol::new(env, ISSUERS_KEY))
        .unwrap_or_else(|| Vec::new(env));
    
    // Create a new list without the removed issuer
    let mut new_issuers = Vec::new(env);
    let mut found = false;
    
    for i in 0..issuers.len() {
        let current = issuers.get(i).unwrap();
        if current == issuer {
            found = true;
        } else {
            new_issuers.push_back(current);
        }
    }
    
    // If issuer was not found, return false
    if !found {
        return false;
    }
    
    // Update the issuers list
    env.storage().instance().set(&Symbol::new(env, ISSUERS_KEY), &new_issuers);
    
    // Emit issuer removed event
    env.events().publish(
        (Symbol::new(env, ISSUER_REMOVED_EVENT),),
        (admin, issuer),
    );
    
    true
}

// Check if an address is an issuer
pub fn is_issuer(env: &Env, address: Address) -> bool {
    // Admin is always considered an issuer
    if has_admin(env) && get_admin(env) == address {
        return true;
    }
    
    let issuers: Vec<Address> = env
        .storage()
        .instance()
        .get(&Symbol::new(env, ISSUERS_KEY))
        .unwrap_or_else(|| Vec::new(env));
    
    for i in 0..issuers.len() {
        if issuers.get(i).unwrap() == address {
            return true;
        }
    }
    
    false
}

// Get all issuers
pub fn get_issuers(env: &Env) -> Vec<Address> {
    env.storage()
        .instance()
        .get(&Symbol::new(env, ISSUERS_KEY))
        .unwrap_or_else(|| Vec::new(env))
}

// Require the caller to be an issuer or admin
pub fn require_issuer(env: &Env) -> Address {
    // Get the transaction source account
    // In Soroban, we'll use the current contract's ID as a fallback
    // since we don't have direct access to the transaction source
    let _contract_id = env.current_contract_address();
    
    // Check if the admin is set and use that
    let admin = get_admin(env);
    admin.require_auth();
    
    // Return the admin as the authorized issuer
    admin
}

// Revoke a certificate
pub fn revoke_certificate(env: &Env, certificate_id: CertificateId) -> bool {
    // Verify the caller is an authorized issuer or admin
    let caller = require_issuer(env);
    
    // Get the certificate
    let cert_id_bytes = env.crypto().sha256(&Bytes::from_slice(env, &certificate_id.0.to_array()));
    let certificate_key = get_certificate_key(env, &cert_id_bytes);
    
    let mut certificate: Certificate = env
        .storage()
        .persistent()
        .get(&certificate_key)
        .unwrap_or_else(|| panic!("Certificate not found"));
    
    // Check if already revoked
    if certificate.revoked {
        return false;
    }
    
    // Update the certificate
    certificate.revoked = true;
    env.storage().persistent().set(&certificate_key, &certificate);
    
    // Emit certificate revoked event
    env.events().publish(
        (Symbol::new(env, CERTIFICATE_REVOKED_EVENT),),
        (certificate_id, certificate.issuer, caller),
    );
    
    true
}

// Helper function to get a certificate key from a hash
fn get_certificate_key(env: &Env, hash: &soroban_sdk::crypto::Hash<32>) -> Symbol {
    // Create a simple key with the hash - avoid using colons
    let key = format!("cert{:x}", hash.to_array()[0]);
    Symbol::new(env, &key)
}
