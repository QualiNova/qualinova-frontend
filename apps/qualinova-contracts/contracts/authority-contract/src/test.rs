#![cfg(test)]
extern crate std;

use crate::types::*;
use crate::{CertifyingAuthority, CertifyingAuthorityClient};
use soroban_sdk::{Address, Env, String, testutils::Address as _, vec};

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&admin);

    assert!(true, "Initialize function completed without errors");
}

#[test]
fn test_register_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![
        &env,
        String::from_str(&env, "ISO_9001"),
        String::from_str(&env, "ISO_13485"),
    ];

    let authority_id = contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    let authority = contract_client.get_authority(&public_key);
    assert_eq!(authority.authority_id, authority_id);
    assert_eq!(
        authority.name,
        String::from_str(&env, "Quality Certification Services Inc.")
    );
    assert_eq!(authority.public_key, public_key);
    assert_eq!(
        authority.accreditation_info,
        String::from_str(&env, "ISO 17021:2015")
    );
    assert_eq!(authority.allowed_cert_types.len(), 2);
    assert_eq!(authority.status, AuthorityStatus::Active);
}

#[test]
#[should_panic(expected = "Authority with this public key already exists")]
fn test_authority_uniqueness() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];

    // Register first authority
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    // Attempt to register another authority with the same public key
    contract_client.register_authority(
        &String::from_str(&env, "Another Authority"),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );
}

#[test]
fn test_update_authority_info() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.update_authority_info(
        &public_key,
        &String::from_str(&env, "name"),
        &String::from_str(&env, "Updated Authority Name"),
    );

    let authority = contract_client.get_authority(&public_key);
    assert_eq!(
        authority.name,
        String::from_str(&env, "Updated Authority Name")
    );
}

#[test]
fn test_deactivate_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.deactivate_authority(&public_key);

    let authority = contract_client.get_authority(&public_key);
    assert_eq!(authority.status, AuthorityStatus::Inactive);
}

#[test]
fn test_add_remove_certification_type() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.add_certification_type(&public_key, &String::from_str(&env, "ISO_13485"));
    let authority = contract_client.get_authority(&public_key);
    assert_eq!(authority.allowed_cert_types.len(), 2);
    assert!(
        authority
            .allowed_cert_types
            .contains(&String::from_str(&env, "ISO_13485"))
    );

    contract_client.remove_certification_type(&public_key, &String::from_str(&env, "ISO_9001"));
    let authority = contract_client.get_authority(&public_key);
    assert_eq!(authority.allowed_cert_types.len(), 1);
    assert!(
        !authority
            .allowed_cert_types
            .contains(&String::from_str(&env, "ISO_9001"))
    );
}

#[test]
#[should_panic(expected = "Name cannot be empty")]
fn test_register_authority_empty_name() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, ""), // Empty name
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );
}

#[test]
#[should_panic(expected = "Accreditation info cannot be empty")]
fn test_register_authority_empty_accreditation_info() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, ""), // Empty accreditation info
        &cert_types,
    );
}

#[test]
fn test_register_authority_empty_cert_types() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env]; // Empty certification types
    let authority_id = contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    let authority = contract_client.get_authority(&public_key);
    assert_eq!(authority.authority_id, authority_id);
    assert_eq!(authority.allowed_cert_types.len(), 0);
}

#[test]
#[should_panic(expected = "Certification types must be unique")]
fn test_register_authority_duplicate_cert_types() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![
        &env,
        String::from_str(&env, "ISO_9001"),
        String::from_str(&env, "ISO_9001"), // Duplicate
    ];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );
}

#[test]
#[should_panic(expected = "Invalid field")]
fn test_update_authority_invalid_field() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.update_authority_info(
        &public_key,
        &String::from_str(&env, "invalid_field"),
        &String::from_str(&env, "Invalid Value"),
    );
}

#[test]
#[should_panic(expected = "Authority not found")]
fn test_update_non_existent_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.update_authority_info(
        &public_key,
        &String::from_str(&env, "name"),
        &String::from_str(&env, "Updated Name"),
    );
}

#[test]
#[should_panic(expected = "Authority not found")]
fn test_deactivate_non_existent_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.deactivate_authority(&public_key);
}

#[test]
#[should_panic(expected = "Authority not found")]
fn test_get_non_existent_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.get_authority(&public_key);
}

#[test]
#[should_panic(expected = "Cannot modify inactive authority")]
fn test_add_cert_type_inactive_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.deactivate_authority(&public_key);
    contract_client.add_certification_type(&public_key, &String::from_str(&env, "ISO_13485"));
}

#[test]
#[should_panic(expected = "Cannot modify inactive authority")]
fn test_remove_cert_type_inactive_authority() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.deactivate_authority(&public_key);
    contract_client.remove_certification_type(&public_key, &String::from_str(&env, "ISO_9001"));
}

#[test]
#[should_panic(expected = "Certification type not found")]
fn test_remove_non_existent_cert_type() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.remove_certification_type(&public_key, &String::from_str(&env, "ISO_13485"));
}

#[test]
#[should_panic(expected = "Contract already initialized")]
fn test_initialize_contract_twice() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&admin);
    contract_client.initialize(&admin);
}

#[test]
#[should_panic(expected = "Authority with this public key already exists")]
fn test_register_authority_after_deactivation() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifyingAuthority, ());
    let contract_client = CertifyingAuthorityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let cert_types = vec![&env, String::from_str(&env, "ISO_9001")];
    contract_client.register_authority(
        &String::from_str(&env, "Quality Certification Services Inc."),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );

    contract_client.deactivate_authority(&public_key);
    contract_client.register_authority(
        &String::from_str(&env, "New Authority"),
        &public_key,
        &String::from_str(&env, "ISO 17021:2015"),
        &cert_types,
    );
}
