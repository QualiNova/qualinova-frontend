#![cfg(test)]
#![allow(dead_code)]

use soroban_sdk::{vec, Env, String};
use crate::CertificationTypeManagementContract;
use client::CertificationTypeManagementContractClient;

#[test]
fn test_register_certification_type() {
    let env = Env::default();
    let contract_id = env.register(CertificationTypeManagementContract, ());
    let client = CertificationTypeManagementContractClient::new(&env, &contract_id);

    let name = String::from_str(&env, "ISO_9001");
    let version = String::from_str(&env, "2015");
    let description = String::from_str(&env, "Quality Management System standard");
    let industry_scope = vec![
        &env,
        String::from_str(&env, "Manufacturing"),
        String::from_str(&env, "Services"),
    ];
    let validity_period = String::from_str(&env, "3 years");

    let cert_type_id = client.register_certification_type(
        &name,
        &version,
        &description,
        &industry_scope,
        &validity_period,
    );

    // Verify the certification type was created
    let cert_type = client.get_certification_type_details(&cert_type_id);
    assert_eq!(cert_type.name, name);
    assert_eq!(cert_type.version, version);
    assert_eq!(cert_type.description, description);
    assert_eq!(cert_type.industry_scope, industry_scope);
    assert_eq!(cert_type.validity_period, validity_period);
}

#[test]
fn test_set_required_evidence() {
    let env = Env::default();
    let contract_id = env.register(CertificationTypeManagementContract, ());
    let client = CertificationTypeManagementContractClient::new(&env, &contract_id);

    // Register a certification type
    let name = String::from_str(&env, "ISO_9001");
    let version = String::from_str(&env, "2015");
    let description = String::from_str(&env, "Quality Management System standard");
    let industry_scope = vec![
        &env,
        String::from_str(&env, "Manufacturing"),
    ];
    let validity_period = String::from_str(&env, "3 years");

    let cert_type_id = client.register_certification_type(
        &name,
        &version,
        &description,
        &industry_scope,
        &validity_period,
    );

    // Set required evidence
    let evidence_list = vec![
        &env,
        String::from_str(&env, "Audit Report"),
        String::from_str(&env, "Conformity Statement"),
    ];
    client.set_required_evidence(&cert_type_id, &evidence_list);

    // Verify the evidence was set
    let cert_type = client.get_certification_type_details(&cert_type_id);
    assert_eq!(cert_type.required_evidence, evidence_list);
}

#[test]
fn test_deprecate_certification_type() {
    let env = Env::default();
    let contract_id = env.register(CertificationTypeManagementContract, ());
    let client = CertificationTypeManagementContractClient::new(&env, &contract_id);

    // Register a certification type
    let name = String::from_str(&env, "ISO_9001");
    let version = String::from_str(&env, "2015");
    let description = String::from_str(&env, "Quality Management System standard");
    let industry_scope = vec![
        &env,
        String::from_str(&env, "Manufacturing"),
    ];
    let validity_period = String::from_str(&env, "3 years");

    let cert_type_id = client.register_certification_type(
        &name,
        &version,
        &description,
        &industry_scope,
        &validity_period,
    );

    // Deprecate the certification type
    let reason = String::from_str(&env, "Replaced by newer version");
    client.deprecate_certification_type(&cert_type_id, &reason);

    // Verify the certification type was deprecated
    let cert_type = client.get_certification_type_details(&cert_type_id);
    assert_eq!(cert_type.status, crate::types::CertTypeStatus::Deprecated);
    assert_eq!(cert_type.deprecation_reason, Some(reason));
}

#[test]
fn test_list_all_certification_types() {
    let env = Env::default();
    let contract_id = env.register(CertificationTypeManagementContract, ());
    let client = CertificationTypeManagementContractClient::new(&env, &contract_id);

    // Register multiple certification types
    let name1 = String::from_str(&env, "ISO_9001");
    let version1 = String::from_str(&env, "2015");
    let description1 = String::from_str(&env, "Quality Management System standard");
    let industry_scope1 = vec![&env, String::from_str(&env, "Manufacturing")];
    let validity_period1 = String::from_str(&env, "3 years");

    let name2 = String::from_str(&env, "ISO_14001");
    let version2 = String::from_str(&env, "2015");
    let description2 = String::from_str(&env, "Environmental Management System standard");
    let industry_scope2 = vec![&env, String::from_str(&env, "Manufacturing")];
    let validity_period2 = String::from_str(&env, "3 years");

    client.register_certification_type(
        &name1,
        &version1,
        &description1,
        &industry_scope1,
        &validity_period1,
    );

    client.register_certification_type(
        &name2,
        &version2,
        &description2,
        &industry_scope2,
        &validity_period2,
    );

    // List all certification types
    let cert_types = client.list_all_certification_types();

    // Verify there are 2 certification types
    assert_eq!(cert_types.len(), 2);
}

#[test]
fn test_multiple_evidence_updates() {
    let env = Env::default();
    let contract_id = env.register(CertificationTypeManagementContract, ());
    let client = CertificationTypeManagementContractClient::new(&env, &contract_id);

    // Register a certification type
    let name = String::from_str(&env, "ISO_9001");
    let version = String::from_str(&env, "2015");
    let description = String::from_str(&env, "Quality Management System standard");
    let industry_scope = vec![
        &env,
        String::from_str(&env, "Manufacturing"),
    ];
    let validity_period = String::from_str(&env, "3 years");

    let cert_type_id = client.register_certification_type(
        &name,
        &version,
        &description,
        &industry_scope,
        &validity_period,
    );

    // Set initial required evidence
    let evidence_list1 = vec![
        &env,
        String::from_str(&env, "Audit Report"),
        String::from_str(&env, "Conformity Statement"),
    ];
    client.set_required_evidence(&cert_type_id, &evidence_list1);

    // Verify the evidence was set
    let cert_type = client.get_certification_type_details(&cert_type_id);
    assert_eq!(cert_type.required_evidence, evidence_list1);

    // Update with new required evidence
    let evidence_list2 = vec![
        &env,
        String::from_str(&env, "Quality Manual"),
        String::from_str(&env, "Process Documentation"),
        String::from_str(&env, "Internal Audit Records"),
    ];
    client.set_required_evidence(&cert_type_id, &evidence_list2);

    // Verify the evidence was updated
    let cert_type = client.get_certification_type_details(&cert_type_id);
    assert_eq!(cert_type.required_evidence, evidence_list2);
    assert_ne!(cert_type.required_evidence, evidence_list1);
}

#[test]
fn test_register_different_versions() {
    let env = Env::default();
    let contract_id = env.register(CertificationTypeManagementContract, ());
    let client = CertificationTypeManagementContractClient::new(&env, &contract_id);

    // Register a certification type with version 2015
    let name1 = String::from_str(&env, "ISO_9001");
    let version1 = String::from_str(&env, "2015");
    let description1 = String::from_str(&env, "Quality Management System standard 2015");
    let industry_scope1 = vec![
        &env,
        String::from_str(&env, "Manufacturing"),
    ];
    let validity_period1 = String::from_str(&env, "3 years");

    let cert_type_id1 = client.register_certification_type(
        &name1,
        &version1,
        &description1,
        &industry_scope1,
        &validity_period1,
    );

    // Register the same certification type but with a different version 2022
    let name2 = String::from_str(&env, "ISO_9001");
    let version2 = String::from_str(&env, "2022");
    let description2 = String::from_str(&env, "Quality Management System standard 2022");
    let industry_scope2 = vec![
        &env,
        String::from_str(&env, "Manufacturing"),
    ];
    let validity_period2 = String::from_str(&env, "5 years");

    let cert_type_id2 = client.register_certification_type(
        &name2,
        &version2,
        &description2,
        &industry_scope2,
        &validity_period2,
    );

    // Verify both certification types were created with different IDs
    assert_ne!(cert_type_id1, cert_type_id2);

    // Get and verify details of the first certification type
    let cert_type1 = client.get_certification_type_details(&cert_type_id1);
    assert_eq!(cert_type1.name, name1);
    assert_eq!(cert_type1.version, version1);
    assert_eq!(cert_type1.description, description1);

    // Get and verify details of the second certification type
    let cert_type2 = client.get_certification_type_details(&cert_type_id2);
    assert_eq!(cert_type2.name, name2);
    assert_eq!(cert_type2.version, version2);
    assert_eq!(cert_type2.description, description2);
}

// We need the client definition for the tests
#[cfg(test)]
mod client {
    use soroban_sdk::{contractclient, Env, String, Vec};
    use crate::CertificationType;

    #[contractclient(name = "CertificationTypeManagementContractClient")]
    pub trait CertificationTypeManagementContractInterface {
        fn register_certification_type(
            env: Env,
            name: String,
            version: String,
            description: String,
            industry_scope: Vec<String>,
            validity_period: String,
        ) -> String;

        fn update_certification_type(
            env: Env,
            cert_type_id: String,
            field: String,
            value: String,
        );

        fn deprecate_certification_type(
            env: Env,
            cert_type_id: String,
            reason: String,
        );

        fn list_all_certification_types(env: Env) -> Vec<CertificationType>;

        fn get_certification_type_details(
            env: Env,
            cert_type_id: String,
        ) -> CertificationType;

        fn set_required_evidence(
            env: Env,
            cert_type_id: String,
            evidence_list: Vec<String>,
        );

        fn assign_authority_to_cert_type(
            env: Env,
            cert_type_id: String,
            authority_id: String,
        );
    }
}