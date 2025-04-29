use soroban_sdk::{Address, String, Vec, contracttype};

#[contracttype]
pub enum DataKey {
    Authorities(Address), // Authority's public key -> Authority data
    AuthorityIds,         // List of all authority IDs (public keys)
    NextAuthorityId,      // Counter for generating unique authority IDs
    Admin,
}

#[contracttype]
#[derive(Clone)]
pub struct Authority {
    pub authority_id: String,            // Unique identifier for the authority
    pub name: String,                    // Name of the authority
    pub public_key: Address,             // Stellar public key of the authority
    pub registration_date: u64,          // Timestamp of registration
    pub accreditation_info: String,      // Accreditation details (e.g., ISO 17021:2015)
    pub allowed_cert_types: Vec<String>, // List of certification types the authority can issue
    pub status: AuthorityStatus,         // Active or Inactive
}

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum AuthorityStatus {
    Active,
    Inactive,
}
