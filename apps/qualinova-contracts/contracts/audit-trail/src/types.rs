use soroban_sdk::{contracttype, Address, BytesN, String};

// Operation type definitions
pub type OperationType = u32;

// Data structures
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct AuditRecord {
    pub audit_id: String,
    pub operation_type: OperationType,
    pub timestamp: u64,
    pub subject_id: String,
    pub performed_by: Address,
    pub operation_details: String,
    pub blockchain_tx_id: BytesN<32>, // 32-byte hash
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct SearchParams {
    pub subject_id: Option<String>,
    pub operation_type: Option<OperationType>,
    pub from_timestamp: Option<u64>,
    pub to_timestamp: Option<u64>,
    pub performed_by: Option<Address>,
}

// Use separate keys for different data types to avoid trait implementation issues
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum DataKey {
    AuditRecord(String), // audit_id
    AuditRecordsByType(OperationType), // Use u32 as OperationType
    AuditRecordsBySubject(String), // subject_id
    LastAuditId, // To store the last assigned ID number
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ExportFormat {
    Json,
    Csv,
    Pdf,
}