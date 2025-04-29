#![no_std]
use soroban_sdk::{
    contract, contractimpl, Env, String, Vec, Map, BytesN, Symbol, Address,
    IntoVal, log
};

mod errors;
mod types;
mod storage;
mod auth;
mod utils;

use errors::Error;
use types::{
    Certification, VerificationReport, VerificationStatus, CertificationStatus,
    DataKey, EvidenceRecord
};
use storage::VerificationStorage;
use auth::AuthHandler;
use utils::crypto;

#[contract]
pub struct VerificationContract;

#[contractimpl]
impl VerificationContract {
    /// Initialize the contract with an admin address and other contract references
    pub fn initialize(
        env: Env, 
        admin: Address, 
        certification_contract_id: Address,
        authority_contract_id: Address
    ) -> Result<(), Error> {
        let storage = VerificationStorage::new(&env);
        
        // Only allow initialization if not already initialized
        if storage.is_initialized() {
            return Err(Error::AlreadyInitialized);
        }
        
        // Set up contract references
        storage.set_admin(admin)?;
        storage.set_certification_contract_id(certification_contract_id)?;
        storage.set_authority_contract_id(authority_contract_id)?;
        
        Ok(())
    }
    
    /// Verifies a certification by its ID
    /// Returns the certification data if it exists and is valid
    pub fn verify_by_id(env: Env, cert_id: String) -> Result<Certification, Error> {
        let storage = VerificationStorage::new(&env);
        let certification = storage.get_certification(&cert_id)?;
        
        // Check certification status
        match certification.status {
            CertificationStatus::Active => Ok(certification),
            CertificationStatus::Revoked => Err(Error::CertificationRevoked),
            CertificationStatus::Suspended => Err(Error::CertificationSuspended),
            CertificationStatus::Expired => Err(Error::CertificationExpired),
        }
    }
    
    /// Verifies the digital signature of a certification
    /// This checks that the signature was created by the authority and matches the certification data
    pub fn check_authentic_signature(
        env: Env, 
        cert_id: String
    ) -> Result<bool, Error> {
        let storage = VerificationStorage::new(&env);
        let auth = AuthHandler::new(&env);
        let certification = storage.get_certification(&cert_id)?;
        
        // Get authority's public key
        let authority_public_key = storage.get_authority_public_key(&certification.authority_id)?;
        
        // Create message to verify (certification data minus the signature)
        let message = crypto::create_certification_message(&env, &certification);
        
        // Verify the signature
        auth.verify_signature(
            &authority_public_key,
            &message,
            &certification.digital_signature
        )
    }
    
    /// Validates if the certification has not expired
    pub fn validate_expiry_date(env: Env, cert_id: String) -> Result<bool, Error> {
        let storage = VerificationStorage::new(&env);
        let certification = storage.get_certification(&cert_id)?;
        
        // Get current timestamp from the ledger
        let current_time = env.ledger().timestamp();
        
        // Check if current time is less than or equal to expiry date
        Ok(current_time <= certification.expiry_date)
    }
    
    /// Validates that the authority is valid for issuing this certification type
    pub fn validate_authority(env: Env, cert_id: String) -> Result<bool, Error> {
        let storage = VerificationStorage::new(&env);
        let certification = storage.get_certification(&cert_id)?;
        
        // Check if the authority is valid for this cert type
        storage.validate_authority_for_cert_type(
            &certification.authority_id, 
            &certification.cert_type
        )
    }
    
    /// Generates a comprehensive verification report for a certification
    pub fn generate_verification_report(env: Env, cert_id: String) -> Result<VerificationReport, Error> {
        let storage = VerificationStorage::new(&env);
        let certification = storage.get_certification(&cert_id)?;
        
        // Perform all verification checks
        let signature_valid = Self::check_authentic_signature(env.clone(), cert_id.clone()).unwrap_or(false);
        let expiry_valid = Self::validate_expiry_date(env.clone(), cert_id.clone()).unwrap_or(false);
        let authority_valid = Self::validate_authority(env.clone(), cert_id.clone()).unwrap_or(false);
        
        // Determine overall verification status
        let status = match certification.status {
            CertificationStatus::Active if signature_valid && expiry_valid && authority_valid => {
                VerificationStatus::Valid
            },
            CertificationStatus::Active if !expiry_valid => VerificationStatus::Expired,
            CertificationStatus::Revoked => VerificationStatus::Revoked,
            CertificationStatus::Suspended => VerificationStatus::Suspended,
            CertificationStatus::Expired => VerificationStatus::Expired,
            _ => VerificationStatus::Invalid,
        };
        
        // Create the verification report
        let verification_report = VerificationReport {
            cert_id: certification.cert_id,
            cert_type: certification.cert_type,
            version: certification.version,
            authority_id: certification.authority_id,
            entity_id: certification.entity_id,
            issue_date: certification.issue_date,
            expiry_date: certification.expiry_date,
            signature_valid,
            expiry_valid,
            authority_valid,
            status,
            verification_timestamp: env.ledger().timestamp(),
        };
        
        Ok(verification_report)
    }
}