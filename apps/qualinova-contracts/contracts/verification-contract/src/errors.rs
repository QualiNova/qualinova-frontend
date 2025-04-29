use soroban_sdk::contracterror;

/// Error codes for the verification contract
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // Contract management errors
    AlreadyInitialized = 1,
    NotInitialized = 2,
    ContractReferenceNotSet = 3,
    
    // Authorization errors
    Unauthorized = 10,
    AdminOnly = 11,
    
    // Certification errors
    CertificationNotFound = 20,
    CertificationInactive = 22,
    CertificationExpired = 23,
    CertificationRevoked = 24,
    CertificationSuspended = 25,
    
    // Verification errors
    InvalidSignature = 30,
    SignatureVerificationFailed = 31,
    InvalidAuthority = 34,
    
    // External contract errors
    ExternalContractError = 40,
    AuthorityContractError = 41,
    CertificationContractError = 42,
}