use soroban_sdk::{contracttype, vec, Env, String, Vec};
use crate::types::{CertTypeStatus, CertificationType};
use crate::error::Error;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    CertificationType(String),  // cert_type_id -> CertificationType
    CertTypeIds,                // -> Vec<String>
    NextId,                     // -> u64
}

pub struct Storage<'a> {
    env: &'a Env,
}

impl<'a> Storage<'a> {
    pub fn new(env: &'a Env) -> Self {
        Self { env }
    }

    // Get all certification type IDs
    fn get_cert_type_ids(&self) -> Vec<String> {
        self.env
            .storage()
            .instance()
            .get(&DataKey::CertTypeIds)
            .unwrap_or_else(|| vec![self.env])
    }

    // Save all certification type IDs
    fn save_cert_type_ids(&self, ids: Vec<String>) {
        self.env.storage().instance().set(&DataKey::CertTypeIds, &ids);
    }

    // Generate a unique ID for a new certification type
    fn generate_cert_type_id(&self) -> String {
        // Get or initialize next ID counter
        let next_id: u64 = self.env
            .storage()
            .instance()
            .get(&DataKey::NextId)
            .unwrap_or(0);

        // Increment the ID counter for next time
        self.env.storage().instance().set(&DataKey::NextId, &(next_id + 1));

        // Use simple ID format with a fixed string
        if next_id == 0 {
            String::from_str(self.env, "CERT_TYPE_A")
        } else if next_id == 1 {
            String::from_str(self.env, "CERT_TYPE_B")
        } else {
            String::from_str(self.env, "CERT_TYPE_C")
        }
    }

    // Register a new certification type
    pub fn register_certification_type(
        &self,
        name: String,
        version: String,
        description: String,
        industry_scope: Vec<String>,
        validity_period: String,
    ) -> Result<String, Error> {
        let cert_type_id = self.generate_cert_type_id();

        // Create the new certification type
        let cert_type = CertificationType::new(
            self.env,
            cert_type_id.clone(),
            name,
            version,
            description,
            industry_scope,
            validity_period,
        );

        // Store the certification type
        self.env.storage().instance().set(
            &DataKey::CertificationType(cert_type_id.clone()),
            &cert_type,
        );

        // Update the list of certification type IDs
        let mut cert_type_ids = self.get_cert_type_ids();
        cert_type_ids.push_front(cert_type_id.clone());
        self.save_cert_type_ids(cert_type_ids);

        Ok(cert_type_id)
    }

    // Update a specific field of a certification type
    pub fn update_certification_type(
        &self,
        cert_type_id: String,
        field: String,
        value: String,
    ) -> Result<(), Error> {
        // Get the certification type
        let mut cert_type = self.get_certification_type_details(cert_type_id.clone())?;

        // Check if the certification type is deprecated
        if cert_type.status == CertTypeStatus::Deprecated {
            return Err(Error::AlreadyDeprecated);
        }

        // Update the specified field
        if field == String::from_str(self.env, "description") {
            cert_type.description = value;
        } else if field == String::from_str(self.env, "verification_requirements") {
            cert_type.verification_requirements = value;
        } else {
            return Err(Error::InvalidField);
        }

        // Save the updated certification type
        self.env.storage().instance().set(
            &DataKey::CertificationType(cert_type_id),
            &cert_type,
        );

        Ok(())
    }

    // Deprecate a certification type
    pub fn deprecate_certification_type(
        &self,
        cert_type_id: String,
        reason: String,
    ) -> Result<(), Error> {
        // Get the certification type
        let mut cert_type = self.get_certification_type_details(cert_type_id.clone())?;

        // Check if the certification type is already deprecated
        if cert_type.status == CertTypeStatus::Deprecated {
            return Err(Error::AlreadyDeprecated);
        }

        // Update the status and deprecation reason
        cert_type.status = CertTypeStatus::Deprecated;
        cert_type.deprecation_reason = Some(reason);

        // Save the updated certification type
        self.env.storage().instance().set(
            &DataKey::CertificationType(cert_type_id),
            &cert_type,
        );

        Ok(())
    }

    // List all certification types
    pub fn list_all_certification_types(&self) -> Vec<CertificationType> {
        let cert_type_ids = self.get_cert_type_ids();
        let mut cert_types = vec![self.env];

        for id in cert_type_ids.iter() {
            if let Some(cert_type) = self.env.storage().instance().get::<DataKey, CertificationType>(
                &DataKey::CertificationType(id.clone())
            ) {
                cert_types.push_front(cert_type);
            }
        }

        cert_types
    }

    // Get details of a specific certification type
    pub fn get_certification_type_details(
        &self,
        cert_type_id: String,
    ) -> Result<CertificationType, Error> {
        self.env.storage().instance().get(&DataKey::CertificationType(cert_type_id.clone()))
            .ok_or(Error::NotFound)
    }

    // Set required evidence for a certification type
    pub fn set_required_evidence(
        &self,
        cert_type_id: String,
        evidence_list: Vec<String>,
    ) -> Result<(), Error> {
        // Get the certification type
        let mut cert_type = self.get_certification_type_details(cert_type_id.clone())?;

        // Check if the certification type is deprecated
        if cert_type.status == CertTypeStatus::Deprecated {
            return Err(Error::AlreadyDeprecated);
        }

        // Update the required evidence
        cert_type.required_evidence = evidence_list;

        // Save the updated certification type
        self.env.storage().instance().set(
            &DataKey::CertificationType(cert_type_id),
            &cert_type,
        );

        Ok(())
    }

    // Assign an authority to a certification type
    pub fn assign_authority_to_cert_type(
        &self,
        cert_type_id: String,
        authority_id: String,
    ) -> Result<(), Error> {
        // Get the certification type
        let mut cert_type = self.get_certification_type_details(cert_type_id.clone())?;

        // Check if the certification type is deprecated
        if cert_type.status == CertTypeStatus::Deprecated {
            return Err(Error::AlreadyDeprecated);
        }

        // Check if the authority is already assigned
        for auth in cert_type.authorities.iter() {
            if auth == authority_id {
                return Err(Error::AlreadyExists);
            }
        }

        // Add the authority
        cert_type.authorities.push_front(authority_id);

        // Save the updated certification type
        self.env.storage().instance().set(
            &DataKey::CertificationType(cert_type_id),
            &cert_type,
        );

        Ok(())
    }
}