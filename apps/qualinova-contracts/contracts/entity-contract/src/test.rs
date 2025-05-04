#![cfg(test)]
extern crate std;

use crate::types::*;
use crate::{CertifiableEntity, CertifiableEntityClient};
use soroban_sdk::{Address, Env, String, testutils::Address as _};

#[test]
fn test_initialize_contract() {
    let env = Env::default();
    let admin = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&admin);

    assert!(true, "Initialize function completed without errors");
}

#[test]
fn test_register_entity() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    let entity_id = contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    let entity = contract_client.get_entity(&public_key);
    assert_eq!(entity.entity_id, entity_id);
    assert_eq!(entity.name, String::from_str(&env, "Acme Manufacturing Corp"));
    assert_eq!(entity.public_key, public_key);
    assert_eq!(entity.industry_sector, String::from_str(&env, "Automotive"));
    assert_eq!(entity.location, String::from_str(&env, "San Francisco, CA"));
    assert_eq!(entity.contact_info, String::from_str(&env, "contact@acmecorp.com"));
    assert_eq!(entity.status, EntityStatus::Active);
    assert_eq!(entity.certifications.len(), 0);
}

#[test]
#[should_panic(expected = "Entity with this public key already exists")]
fn test_entity_uniqueness() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    // Register first entity
    contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    // Attempt to register another entity with the same public key
    contract_client.register_entity(
        &String::from_str(&env, "Another Entity"),
        &String::from_str(&env, "Technology"),
        &String::from_str(&env, "New York, NY"),
        &String::from_str(&env, "contact@another.com"),
        &public_key,
    );
}

#[test]
fn test_update_entity_info() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    contract_client.update_entity_info(
        &public_key,
        &String::from_str(&env, "name"),
        &String::from_str(&env, "Updated Company Name"),
    );

    let entity = contract_client.get_entity(&public_key);
    assert_eq!(entity.name, String::from_str(&env, "Updated Company Name"));
}

#[test]
#[should_panic(expected = "Invalid field")]
fn test_update_entity_invalid_field() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    contract_client.update_entity_info(
        &public_key,
        &String::from_str(&env, "invalid_field"),
        &String::from_str(&env, "Invalid Value"),
    );
}

#[test]
fn test_verify_entity() {
    let env = Env::default();
    let public_key = Address::generate(&env);
    let non_existent_entity = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    // Verify existing active entity
    assert!(contract_client.verify_entity(&public_key));

    // Verify non-existent entity
    assert!(!contract_client.verify_entity(&non_existent_entity));

    // Deactivate entity
    contract_client.deactivate_entity(&public_key, &String::from_str(&env, "Testing deactivation"));

    // Verify deactivated entity
    assert!(!contract_client.verify_entity(&public_key));
}

#[test]
fn test_deactivate_entity() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    contract_client.deactivate_entity(&public_key, &String::from_str(&env, "Business closure"));

    let entity = contract_client.get_entity(&public_key);
    assert_eq!(entity.status, EntityStatus::Inactive);
}

#[test]
#[should_panic(expected = "Entity already inactive")]
fn test_deactivate_inactive_entity() {
    let env = Env::default();
    let public_key = Address::generate(&env);

    let contract_address = env.register(CertifiableEntity, ());
    let contract_client = CertifiableEntityClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.initialize(&Address::generate(&env));

    contract_client.register_entity(
        &String::from_str(&env, "Acme Manufacturing Corp"),
        &String::from_str(&env, "Automotive"),
        &String::from_str(&env, "San Francisco, CA"),
        &String::from_str(&env, "contact@acmecorp.com"),
        &public_key,
    );

    // First deactivation
    contract_client.deactivate_entity(&public_key, &String::from_str(&env, "Business closure"));

    // Second deactivation - should panic
    contract_client.deactivate_entity(&public_key, &String::from_str(&env, "Another reason"));
}