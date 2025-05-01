use crate::{access_control, Certificate, CertificateId, CertificateMetadata};
use alloc::format;
use soroban_sdk::{Address, Bytes, Env, Symbol, Vec};
use soroban_sdk::xdr::ToXdr;

// Storage keys format
const CERTIFICATE_COUNT_KEY: &str = "cert_count";
const OWNER_CERTS_PREFIX: &str = "owner_certs";
const ISSUER_CERTS_PREFIX: &str = "issuer_certs";

// Events
const CERTIFICATE_ISSUED_EVENT: &str = "certificate_issued";
const CERTIFICATES_BATCH_ISSUED_EVENT: &str = "certificates_batch_issued";

// Issue a new certificate
pub fn issue_certificate(
    env: &Env,
    owner: Address,
    metadata: CertificateMetadata,
    expiration_date: Option<u64>,
    signature: Bytes,
) -> CertificateId {
    // Verify the caller is an authorized issuer
    let issuer = access_control::require_issuer(env);
    
    // Require authorization from the owner
    owner.require_auth();
    
    // Generate a unique certificate ID
    let cert_id = generate_certificate_id(env, &owner, &issuer, &metadata);
    
    // Create the certificate
    let certificate = Certificate {
        id: cert_id.clone(),
        owner: owner.clone(),
        issuer: issuer.clone(),
        metadata: metadata.clone(),
        issuance_date: env.ledger().timestamp(),
        expiration_date,
        revoked: false,
        signature,
    };
    
    // Store the certificate
    let cert_id_bytes = env.crypto().sha256(&Bytes::from_slice(env, &cert_id.0.to_array()));
    let certificate_key = get_certificate_key(env, &cert_id_bytes);
    env.storage().persistent().set(&certificate_key, &certificate);
    
    // Update certificate counts
    increment_certificate_count(env);
    
    // Add to owner's certificates
    add_to_owner_certificates(env, &owner, &cert_id);
    
    // Add to issuer's certificates
    add_to_issuer_certificates(env, &issuer, &cert_id);
    
    // Emit certificate issued event
    env.events().publish(
        (Symbol::new(env, CERTIFICATE_ISSUED_EVENT),),
        (cert_id.clone(), owner, issuer),
    );
    
    cert_id
}

// Batch issue multiple certificates
pub fn batch_issue_certificates(
    env: &Env,
    owners: Vec<Address>,
    metadatas: Vec<CertificateMetadata>,
    expiration_dates: Vec<Option<u64>>,
    signatures: Vec<Bytes>,
) -> Vec<CertificateId> {
    // Verify the caller is an authorized issuer
    let issuer = access_control::require_issuer(env);
    
    // Validate input arrays have the same length
    let count = owners.len();
    if metadatas.len() != count || expiration_dates.len() != count || signatures.len() != count {
        panic!("Input arrays must have the same length");
    }
    
    let mut certificate_ids = Vec::new(env);
    
    // Process each certificate
    for i in 0..count {
        let owner = owners.get(i).unwrap();
        let metadata = metadatas.get(i).unwrap();
        let expiration_date = expiration_dates.get(i).unwrap();
        let signature = signatures.get(i).unwrap();
        
        // Require authorization from each owner
        owner.require_auth();
        
        // Generate a unique certificate ID
        let cert_id = generate_certificate_id(env, &owner, &issuer, &metadata);
        
        // Create the certificate
        let certificate = Certificate {
            id: cert_id.clone(),
            owner: owner.clone(),
            issuer: issuer.clone(),
            metadata: metadata.clone(),
            issuance_date: env.ledger().timestamp(),
            expiration_date,
            revoked: false,
            signature,
        };
        
        // Store the certificate
        let cert_id_bytes = env.crypto().sha256(&Bytes::from_slice(env, &cert_id.0.to_array()));
        let certificate_key = get_certificate_key(env, &cert_id_bytes);
        env.storage().persistent().set(&certificate_key, &certificate);
        
        // Update certificate counts
        increment_certificate_count(env);
        
        // Add to owner's certificates
        add_to_owner_certificates(env, &owner, &cert_id);
        
        // Add to issuer's certificates
        add_to_issuer_certificates(env, &issuer, &cert_id);
        
        // Add to the result list
        certificate_ids.push_back(cert_id);
    }
    
    // Emit batch issued event
    env.events().publish(
        (Symbol::new(env, CERTIFICATES_BATCH_ISSUED_EVENT),),
        (certificate_ids.clone(), issuer),
    );
    
    certificate_ids
}

// Generate a unique certificate ID
pub fn generate_certificate_id(
    env: &Env,
    owner: &Address,
    issuer: &Address,
    metadata: &CertificateMetadata,
) -> CertificateId {
    // Create a combined data structure to hash
    let mut data = Bytes::new(env);
    
    // Add owner and issuer XDR
    data.append(&owner.clone().to_xdr(env));
    data.append(&issuer.clone().to_xdr(env));
    
    // Add metadata - convert strings to bytes using to_xdr
    data.append(&metadata.title.clone().to_xdr(env));
    data.append(&metadata.description.clone().to_xdr(env));
    data.append(&metadata.achievement_type.clone().to_xdr(env));
    
    // Add timestamp
    let timestamp = env.ledger().timestamp();
    // Convert u64 to bytes manually
    let mut tmp = timestamp;
    for _ in 0..8 {
        let byte = (tmp & 0xFF) as u8;
        tmp >>= 8;
        data.append(&Bytes::from_slice(env, &[byte]));
    }
    
    // Add a unique component
    // Instead of using random, use a combination of env data
    let ledger_seq = env.ledger().sequence();
    let ledger_timestamp = env.ledger().timestamp();
    let unique_bytes = Bytes::from_slice(env, &[
        ((ledger_seq >> 24) & 0xFF) as u8,
        ((ledger_seq >> 16) & 0xFF) as u8,
        ((ledger_seq >> 8) & 0xFF) as u8,
        (ledger_seq & 0xFF) as u8,
        ((ledger_timestamp >> 24) & 0xFF) as u8,
        ((ledger_timestamp >> 16) & 0xFF) as u8,
        ((ledger_timestamp >> 8) & 0xFF) as u8,
        (ledger_timestamp & 0xFF) as u8,
    ]);
    data.append(&unique_bytes);
    
    // Create a hash of all the data
    let hash = env.crypto().sha256(&data);
    
    CertificateId(hash.into())
}

// Increment the total certificate count
fn increment_certificate_count(env: &Env) {
    let count_key = Symbol::new(env, CERTIFICATE_COUNT_KEY);
    let current_count: u32 = env.storage().instance().get(&count_key).unwrap_or(0);
    env.storage().instance().set(&count_key, &(current_count + 1));
}

// Get the total certificate count
pub fn get_certificate_count(env: &Env) -> u32 {
    let count_key = Symbol::new(env, CERTIFICATE_COUNT_KEY);
    env.storage().instance().get(&count_key).unwrap_or(0)
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

// Add a certificate to the issuer's list
fn add_to_issuer_certificates(env: &Env, issuer: &Address, cert_id: &CertificateId) {
    let issuer_key = get_issuer_key(env, issuer);
    
    let mut issuer_certs: Vec<CertificateId> = env
        .storage()
        .persistent()
        .get(&issuer_key)
        .unwrap_or_else(|| Vec::new(env));
    
    issuer_certs.push_back(cert_id.clone());
    env.storage().persistent().set(&issuer_key, &issuer_certs);
}

// Get all certificates for an owner
pub fn get_owner_certificates(env: &Env, owner: &Address) -> Vec<CertificateId> {
    let owner_key = get_owner_key(env, owner);
    
    env.storage()
        .persistent()
        .get(&owner_key)
        .unwrap_or_else(|| Vec::new(env))
}

// Helper function to get a certificate key from a hash
fn get_certificate_key(env: &Env, hash: &soroban_sdk::crypto::Hash<32>) -> Symbol {
    // Create a simple key with the hash - avoid using colons or other special characters
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

// Helper function to get an issuer key
fn get_issuer_key(env: &Env, issuer: &Address) -> Symbol {
    let issuer_hash = env.crypto().sha256(&issuer.clone().to_xdr(env));
    // Avoid using colons in Symbol keys
    let key = format!("{}_{:x}", ISSUER_CERTS_PREFIX, issuer_hash.to_array()[0]);
    Symbol::new(env, &key)
}
