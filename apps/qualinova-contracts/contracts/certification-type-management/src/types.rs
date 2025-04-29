use soroban_sdk::{contracttype, vec, Env, String, Vec};

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum CertTypeStatus {
    Active,
    Deprecated,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct CertificationType {
    pub cert_type_id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub industry_scope: Vec<String>,
    pub validity_period: String,
    pub required_evidence: Vec<String>,
    pub verification_requirements: String,
    pub status: CertTypeStatus,
    pub authorities: Vec<String>,
    pub deprecation_reason: Option<String>,
}

impl CertificationType {
    pub fn new(
        env: &Env,
        cert_type_id: String,
        name: String,
        version: String,
        description: String,
        industry_scope: Vec<String>,
        validity_period: String,
    ) -> Self {
        CertificationType {
            cert_type_id,
            name,
            version,
            description,
            industry_scope,
            validity_period,
            required_evidence: vec![env],
            verification_requirements: String::from_str(env, ""),
            status: CertTypeStatus::Active,
            authorities: vec![env],
            deprecation_reason: None,
        }
    }
}