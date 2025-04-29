use soroban_sdk::{Env, BytesN};
use crate::types::Certification;

pub mod crypto {
    use super::*;

    /// Creates a message for signature verification from a certification
    /// The message includes all certification data except the signature itself
    pub fn create_certification_message(env: &Env, certification: &Certification) -> Vec<u8> {
        // Concatenate certification fields in a deterministic order to create the message
        // that was originally signed
        let mut message = Vec::new();

        // Add certification ID
        message.extend_from_slice(certification.cert_id.as_bytes());
        message.push(0u8); // separator

        // Add certification type
        message.extend_from_slice(certification.cert_type.as_bytes());
        message.push(0u8);

        // Add version
        message.extend_from_slice(certification.version.as_bytes());
        message.push(0u8);

        // Add authority ID (as bytes)
        let authority_bytes = certification.authority_id.to_string().as_bytes();
        message.extend_from_slice(authority_bytes);
        message.push(0u8);

        // Add entity ID (as bytes)
        let entity_bytes = certification.entity_id.to_string().as_bytes();
        message.extend_from_slice(entity_bytes);
        message.push(0u8);

        // Add issue date (as bytes)
        let issue_date_bytes = certification.issue_date.to_be_bytes();
        message.extend_from_slice(&issue_date_bytes);

        // Add expiry date (as bytes)
        let expiry_date_bytes = certification.expiry_date.to_be_bytes();
        message.extend_from_slice(&expiry_date_bytes);

        // Add scope
        message.extend_from_slice(certification.scope.as_bytes());
        message.push(0u8);

        // Add evidence hash (using all bytes of the BytesN<32>)
        for i in 0..32 {
            message.push(certification.evidence_hash.get(i));
        }

        message
    }

    /// Generates a hash for evidence documents
    pub fn hash_evidence(env: &Env, evidence_data: &[u8]) -> BytesN<32> {
        env.crypto().sha256(evidence_data)
    }

    /// Validates that a hash matches the expected value
    pub fn validate_hash(actual_hash: &BytesN<32>, expected_hash: &BytesN<32>) -> bool {
        actual_hash == expected_hash
    }
}