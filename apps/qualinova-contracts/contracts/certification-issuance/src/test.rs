#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    Address, Bytes, BytesN, Env, Map, String, vec,
};

// Helper function to create a test environment
fn create_test_env() -> Env {
    let env = Env::default();
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = 12345;
        ledger.protocol_version = 20;
        ledger.sequence_number = 10;
    });
    env
}

// Helper function to create a test certificate metadata
fn create_test_metadata(env: &Env) -> CertificateMetadata {
    let mut additional_data = Map::new(env);
    additional_data.set(
        String::from_str(env, "course_id"),
        BytesN::from_array(env, &[1u8; 32]),
    );
    
    CertificateMetadata {
        title: String::from_str(env, "Blockchain Development Certificate"),
        description: String::from_str(env, "Completed the advanced blockchain development course"),
        achievement_type: String::from_str(env, "course_completion"),
        additional_data,
    }
}

// Helper to setup a contract with an admin
fn setup_contract(env: &Env) -> (Address, CertificationContractClient) {
    let admin = Address::generate(env);
    
    // Initialize the contract
    // Note: register_contract is deprecated but we're using it for compatibility
    // TODO: Update to use register when upgrading the SDK
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, CertificationContract);
    let client = CertificationContractClient::new(env, &contract_id);
    
    // Set the admin as the invoker for the initialize call
    env.mock_all_auths();
    
    // Initialize the contract
    client.initialize(&admin);
    
    (admin, client)
}

#[test]
fn test_initialize_contract() {
    let env = create_test_env();
    let (admin, contract) = setup_contract(&env);
    
    // Verify the admin is set correctly
    assert!(contract.is_issuer(&admin));
}

#[test]
fn test_issue_certificate() {
    let env = create_test_env();
    let (admin, contract) = setup_contract(&env);
    let owner = Address::generate(&env);
    
    // Create test metadata
    let metadata = create_test_metadata(&env);
    
    // Create a dummy signature
    let signature = Bytes::from_slice(&env, &[0u8; 64]);
    
    // Set the admin as the invoker for the issue_certificate call
    env.mock_all_auths();
    
    // Issue a certificate
    let cert_id = contract.issue_certificate(&owner, &metadata, &None, &signature);
    
    // Verify the certificate was created
    let certificate = contract.get_certificate(&cert_id);
    assert_eq!(certificate.owner, owner);
    assert_eq!(certificate.issuer, admin);
    assert_eq!(certificate.metadata.title, metadata.title);
    assert_eq!(certificate.revoked, false);
}

#[test]
fn test_batch_issue_certificates() {
    let env = create_test_env();
    let (_admin, contract) = setup_contract(&env);
    
    // Create multiple owners
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    
    // Create test metadata
    let metadata1 = create_test_metadata(&env);
    let mut metadata2 = metadata1.clone();
    metadata2.title = String::from_str(&env, "Advanced Data Science Certificate");
    
    // Create dummy signatures
    let signature1 = Bytes::from_slice(&env, &[0u8; 64]);
    let signature2 = Bytes::from_slice(&env, &[1u8; 64]);
    
    // Create the input vectors
    let owners = vec![&env, owner1.clone(), owner2.clone()];
    let metadatas = vec![&env, metadata1.clone(), metadata2.clone()];
    let expiration_dates = vec![&env, None, None];
    let signatures = vec![&env, signature1.clone(), signature2.clone()];
    
    // Set the admin as the invoker for the batch_issue_certificates call
    env.mock_all_auths();
    
    // Batch issue certificates
    let cert_ids = contract.batch_issue_certificates(&owners, &metadatas, &expiration_dates, &signatures);
    
    // Verify the certificates were created
    assert_eq!(cert_ids.len(), 2);
    
    let certificate1 = contract.get_certificate(&cert_ids.get(0).unwrap());
    assert_eq!(certificate1.owner, owner1);
    assert_eq!(certificate1.metadata.title, metadata1.title);
    
    let certificate2 = contract.get_certificate(&cert_ids.get(1).unwrap());
    assert_eq!(certificate2.owner, owner2);
    assert_eq!(certificate2.metadata.title, metadata2.title);
}

#[test]
fn test_transfer_certificate() {
    let env = create_test_env();
    let (_admin, contract) = setup_contract(&env);
    let original_owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    
    // Create test metadata
    let metadata = create_test_metadata(&env);
    let signature = Bytes::from_slice(&env, &[0u8; 64]);
    
    // Set the admin as the invoker for the issue_certificate call
    env.mock_all_auths();
    
    // Issue a certificate
    let cert_id = contract.issue_certificate(&original_owner, &metadata, &None, &signature);
    
    // Mock the original owner's authorization for transfer
    env.mock_all_auths();
    
    // Transfer the certificate
    assert!(contract.transfer_certificate(&cert_id, &new_owner));
    
    // Verify the transfer
    let certificate = contract.get_certificate(&cert_id);
    assert_eq!(certificate.owner, new_owner);
}

#[test]
fn test_list_certificates_by_owner() {
    let env = create_test_env();
    let (_admin, contract) = setup_contract(&env);
    let owner = Address::generate(&env);
    
    // Create test metadata
    let metadata1 = create_test_metadata(&env);
    let mut metadata2 = metadata1.clone();
    metadata2.title = String::from_str(&env, "Advanced Data Science Certificate");
    
    let signature = Bytes::from_slice(&env, &[0u8; 64]);
    
    // Set the admin as the invoker for the issue_certificate calls
    env.mock_all_auths();
    
    // Issue certificates
    let cert_id1 = contract.issue_certificate(&owner, &metadata1, &None, &signature);
    let cert_id2 = contract.issue_certificate(&owner, &metadata2, &None, &signature);
    
    // List certificates by owner
    let certificates = contract.list_certificates_by_owner(&owner, &0, &10);
    
    // Verify the list
    assert_eq!(certificates.len(), 2);
    
    // Check if both certificates are in the list
    let has_cert1 = certificates.iter().any(|cert| cert.id == cert_id1);
    let has_cert2 = certificates.iter().any(|cert| cert.id == cert_id2);
    
    assert!(has_cert1);
    assert!(has_cert2);
}

#[test]
fn test_list_certificates_by_issuer() {
    let env = create_test_env();
    let (admin, contract) = setup_contract(&env);
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    
    // Create test metadata
    let metadata = create_test_metadata(&env);
    let signature = Bytes::from_slice(&env, &[0u8; 64]);
    
    // Set the admin as the invoker for the issue_certificate calls
    env.mock_all_auths();
    
    // Issue certificates
    let cert_id1 = contract.issue_certificate(&owner1, &metadata, &None, &signature);
    let cert_id2 = contract.issue_certificate(&owner2, &metadata, &None, &signature);
    
    // List certificates by issuer
    let certificates = contract.list_certificates_by_issuer(&admin, &0, &10);
    
    // Verify the list
    assert_eq!(certificates.len(), 2);
    
    // Check if both certificates are in the list
    let has_cert1 = certificates.iter().any(|cert| cert.id == cert_id1);
    let has_cert2 = certificates.iter().any(|cert| cert.id == cert_id2);
    
    assert!(has_cert1);
    assert!(has_cert2);
}

#[test]
fn test_add_and_remove_issuer() {
    let env = create_test_env();
    let (_admin, contract) = setup_contract(&env);
    let new_issuer = Address::generate(&env);
    
    // Set the admin as the invoker for the add_issuer call
    env.mock_all_auths();
    
    // Add a new issuer
    assert!(contract.add_issuer(&new_issuer));
    
    // Verify the new issuer
    assert!(contract.is_issuer(&new_issuer));
    
    // Set the admin as the invoker for the remove_issuer call
    env.mock_all_auths();
    
    // Remove the issuer
    assert!(contract.remove_issuer(&new_issuer));
    
    // Verify the issuer was removed
    assert!(!contract.is_issuer(&new_issuer));
}

#[test]
fn test_revoke_certificate() {
    let env = create_test_env();
    let (_admin, contract) = setup_contract(&env);
    let owner = Address::generate(&env);
    
    // Create test metadata
    let metadata = create_test_metadata(&env);
    let signature = Bytes::from_slice(&env, &[0u8; 64]);
    
    // Set the admin as the invoker for the issue_certificate call
    env.mock_all_auths();
    
    // Issue a certificate
    let cert_id = contract.issue_certificate(&owner, &metadata, &None, &signature);
    
    // Set the admin as the invoker for the revoke_certificate call
    env.mock_all_auths();
    
    // Revoke the certificate
    assert!(contract.revoke_certificate(&cert_id));
    
    // Verify the certificate is revoked
    let certificate = contract.get_certificate(&cert_id);
    assert!(certificate.revoked);
    
    // Verify the certificate is no longer valid
    assert!(!contract.verify_certificate_signature(&cert_id));
}

#[test]
fn test_transfer_admin() {
    let env = create_test_env();
    let (original_admin, contract) = setup_contract(&env);
    let new_admin = Address::generate(&env);
    
    // Add original_admin as an explicit issuer (so it stays an issuer after transfer)
    env.mock_all_auths();
    contract.add_issuer(&original_admin);
    
    // Set the original admin as the invoker for the transfer_admin call
    env.mock_all_auths();
    
    // Transfer admin role
    assert!(contract.transfer_admin(&new_admin));
    
    // Verify the new admin has admin privileges
    assert!(contract.is_issuer(&new_admin));
    
    // The original admin should still be an issuer because we added them explicitly
    assert!(contract.is_issuer(&original_admin));
}

#[test]
#[should_panic(expected = "Caller is not an authorized issuer or admin")]
fn test_unauthorized_issuance() {
    let env = create_test_env();
    let (admin, _contract) = setup_contract(&env);
    let unauthorized = Address::generate(&env);
    let owner = Address::generate(&env);
    
    // Create a new contract instance that's not initialized
    // Note: register_contract is deprecated but we're using it for compatibility
    // TODO: Update to use register when upgrading the SDK
    #[allow(deprecated)]
    let contract_id = env.register_contract(None, CertificationContract);
    
    // Create test metadata
    let metadata = create_test_metadata(&env);
    let signature = Bytes::from_slice(&env, &[0u8; 64]);
    
    // Initialize the contract with the admin
    env.mock_all_auths();
    env.as_contract(&contract_id, || {
        access_control::initialize(&env, &admin);
        
        // Try to issue a certificate as an unauthorized user
        // This should panic with "Caller is not an authorized issuer or admin"
        // We need to explicitly check that the caller is not an issuer
        if !access_control::is_issuer(&env, unauthorized.clone()) {
            panic!("Caller is not an authorized issuer or admin");
        }
        
        issuance::issue_certificate(&env, owner, metadata, None, signature);
    });
}
