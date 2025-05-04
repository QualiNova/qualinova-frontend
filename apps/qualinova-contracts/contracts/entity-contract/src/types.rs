use soroban_sdk::{Address, String, Vec, contracttype};

#[contracttype]
pub enum DataKey {
    Entities(Address),     // Entity's public key -> Entity data
    EntityIds,             // List of all entity IDs (public keys)
    NextEntityId,          // Counter for generating unique entity IDs
    Admin,                 // Admin address
}

#[contracttype]
#[derive(Clone)]
pub struct Entity {
    pub entity_id: String,             // Unique identifier for the entity
    pub name: String,                  // Name of the organization
    pub public_key: Address,           // Stellar public key of the entity
    pub registration_date: u64,        // Timestamp of registration
    pub industry_sector: String,       // The industry sector (e.g., "Automotive")
    pub location: String,              // Location (e.g., "San Francisco, CA")
    pub contact_info: String,          // Contact information (e.g., email)
    pub status: EntityStatus,          // Active, Inactive, etc.
    pub certifications: Vec<String>,   // List of certification IDs associated with this entity
}

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum EntityStatus {
    Active,
    Inactive,
}