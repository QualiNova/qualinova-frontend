use soroban_sdk::{contracttype, BytesN, Env, String, Symbol, Address};

/// Certification data structure matching the requirements
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certification {
    pub cert_id: String,
    pub cert_type: String,
    pub version: String,
    pub authority_id: Address,
    pub entity_id: Address,
    pub issue_date: u64,    // Unix timestamp
    pub expiry_date: u64,   // Unix timestamp
    pub scope: String,
    pub status: CertificationStatus,
    pub evidence_hash: BytesN<32>,
    pub digital_signature: BytesN<64>,
}

/// Certification status enum
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CertificationStatus {
    Active,
    Revoked,
    Suspended,
    Expired,
}

/// Verification status enum to represent the result of certification verification
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationStatus {
    Valid,
    Invalid,
    Expired,
    Revoked,
    Suspended,
}

/// Verification report structure containing detailed verification results
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationReport {
    pub cert_id: String,
    pub cert_type: String,
    pub version: String,
    pub authority_id: Address,
    pub entity_id: Address,
    pub issue_date: u64,
    pub expiry_date: u64,
    pub signature_valid: bool,
    pub expiry_valid: bool,
    pub authority_valid: bool,
    pub status: VerificationStatus,
    pub verification_timestamp: u64,
}

/// Storage keys for contract data
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    AuthorityContractId,
    CertificationContractId,
    AuthorityPublicKeys(Address),
    AuthorityCertTypes(Address),
}

/// Authority Information returned from the Authority contract
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorityInfo {
    pub id: Address,
    pub name: String,
    pub public_key: BytesN<32>,
    pub accreditation_info: String,
    pub allowed_cert_types: Vec<String>,
    pub status: AuthorityStatus,
}

/// Authority status enum
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityStatus {
    Active,
    Inactive,
}