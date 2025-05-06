use soroban_sdk::{Env, Vec, Map, BytesN, Address, String, IntoVal, FromVal, Val, contractclient};
use crate::errors::Error;
use crate::types::{Certification, DataKey, CertificationStatus};

// Define an interface for the Authority Management Contract
// Updated to match the actual implementation
#[contractclient(name = "AuthorityClient")]
pub trait AuthorityContract {
    fn verify_authority(env: Env, authority_id: Address) -> bool;
    fn get_authority(env: Env, authority_id: Address) -> Val;
    fn get_authority_info(env: Env, authority_id: Address) -> Val;
    fn is_authority_allowed_cert_type(env: Env, authority_id: Address, cert_type: String) -> bool;
}

// Define an interface for the Certification Management Contract
#[contractclient(name = "CertificationClient")]
pub trait CertificationContract {
    fn get_certification(env: Env, cert_id: String) -> Certification;
    fn certification_exists(env: Env, cert_id: String) -> bool;
}

/// Storage handler for the verification contract
pub struct VerificationStorage<'a> {
    env: &'a Env,
}

impl<'a> VerificationStorage<'a> {
    /// Creates a new storage handler
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }

    /// Checks if the contract has been initialized
    pub fn is_initialized(&self) -> bool {
        self.env.storage().instance().has(&DataKey::Admin)
    }

    /// Gets the contract admin
    pub fn get_admin(&self) -> Result<Address, Error> {
        if !self.is_initialized() {
            return Err(Error::NotInitialized);
        }

        self.env.storage().instance().get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    /// Sets the contract admin (only during initialization)
    pub fn set_admin(&self, admin: Address) -> Result<(), Error> {
        if self.is_initialized() {
            return Err(Error::AlreadyInitialized);
        }

        self.env.storage().instance().set(&DataKey::Admin, &admin);
        Ok(())
    }

    /// Gets the authority management contract ID
    pub fn get_authority_contract_id(&self) -> Result<Address, Error> {
        self.env.storage().instance().get(&DataKey::AuthorityContractId)
            .ok_or(Error::ContractReferenceNotSet)
    }

    /// Sets the authority management contract ID
    pub fn set_authority_contract_id(&self, contract_id: Address) -> Result<(), Error> {
        self.env.storage().instance().set(&DataKey::AuthorityContractId, &contract_id);
        Ok(())
    }
    
    /// Gets the certification management contract ID
    pub fn get_certification_contract_id(&self) -> Result<Address, Error> {
        self.env.storage().instance().get(&DataKey::CertificationContractId)
            .ok_or(Error::ContractReferenceNotSet)
    }

    /// Sets the certification management contract ID
    pub fn set_certification_contract_id(&self, contract_id: Address) -> Result<(), Error> {
        self.env.storage().instance().set(&DataKey::CertificationContractId, &contract_id);
        Ok(())
    }

    /// Gets a certification by ID from the certification contract
    pub fn get_certification(&self, cert_id: &String) -> Result<Certification, Error> {
        let cert_contract_id = self.get_certification_contract_id()?;
        let client = CertificationClient::new(self.env, &cert_contract_id);
        
        // Try to get the certification from the certification contract
        match std::panic::catch_unwind(|| {
            client.get_certification(cert_id.clone())
        }) {
            Ok(certification) => Ok(certification),
            Err(_) => Err(Error::CertificationNotFound)
        }
    }

    /// Checks if a certification exists
    pub fn certification_exists(&self, cert_id: &String) -> Result<bool, Error> {
        let cert_contract_id = self.get_certification_contract_id()?;
        let client = CertificationClient::new(self.env, &cert_contract_id);
        
        // Try to check if the certification exists in the certification contract
        match std::panic::catch_unwind(|| {
            client.certification_exists(cert_id.clone())
        }) {
            Ok(exists) => Ok(exists),
            Err(_) => Err(Error::ExternalContractError)
        }
    }

    /// Gets the authority public key from the authority management contract
    pub fn get_authority_public_key(&self, authority_id: &Address) -> Result<BytesN<32>, Error> {
        let authority_contract_id = self.get_authority_contract_id()?;
        let client = AuthorityClient::new(self.env, &authority_contract_id);
        
        // First try to get authority info which should contain the public key
        match std::panic::catch_unwind(|| {
            let authority_info = client.get_authority_info(authority_id.clone());
            
            // Extract public key from authority info (the structure depends on the actual implementation)
            // This is a simplified example - adjust based on the actual authority info structure
            let public_key_val = authority_info.get(&Symbol::from_str(self.env, "public_key")).unwrap_or_else(|| {
                panic!("Public key not found in authority info")
            });
            
            // Convert to BytesN<32>
            BytesN::<32>::from_val(self.env, &public_key_val)
        }) {
            Ok(public_key) => Ok(public_key),
            Err(_) => {
                // If we can't get it from get_authority_info, try directly from storage
                // This is a fallback path and might not be needed if authority contract is properly set up
                Err(Error::InvalidAuthority)
            }
        }
    }

    /// Check if the authority is valid for the given certificate type
    pub fn validate_authority_for_cert_type(&self, authority_id: &Address, cert_type: &String) -> Result<bool, Error> {
        let authority_contract_id = self.get_authority_contract_id()?;
        let client = AuthorityClient::new(self.env, &authority_contract_id);
        
        // Try to call the external contract to validate the cert type
        match std::panic::catch_unwind(|| {
            client.is_authority_allowed_cert_type(authority_id.clone(), cert_type.clone())
        }) {
            Ok(is_valid) => Ok(is_valid),
            Err(_) => {
                // Alternative approach - try to verify authority first
                match std::panic::catch_unwind(|| {
                    client.verify_authority(authority_id.clone())
                }) {
                    Ok(is_valid) => {
                        if !is_valid {
                            return Ok(false);
                        }
                        
                        // If authority is valid, try to get authority info and check cert types
                        let authority_info = client.get_authority(authority_id.clone());
                        
                        // Extract allowed cert types from authority info
                        // This depends on the actual structure of authority_info
                        let cert_types_val = authority_info.get(&Symbol::from_str(self.env, "allowed_cert_types")).unwrap_or_else(|| {
                            panic!("Certification types not found in authority info")
                        });
                        
                        // Convert to Vec<String> and check if it contains the requested cert type
                        let cert_types = Vec::<String>::from_val(self.env, &cert_types_val);
                        Ok(cert_types.contains(cert_type))
                    },
                    Err(_) => Err(Error::ExternalContractError)
                }
            }
        }
    }
}