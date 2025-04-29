use soroban_sdk::{Address, BytesN, Env, IntoVal};
use crate::errors::Error;
use crate::storage::VerificationStorage;

/// Authentication and authorization handler for the verification contract
pub struct AuthHandler<'a> {
    env: &'a Env,
    storage: VerificationStorage<'a>,
}

impl<'a> AuthHandler<'a> {
    /// Creates a new auth handler
    pub fn new(env: &'a Env) -> Self {
        Self {
            env,
            storage: VerificationStorage::new(env),
        }
    }

    /// Checks if the contract is initialized
    pub fn require_initialized(&self) -> Result<(), Error> {
        if !self.storage.is_initialized() {
            return Err(Error::NotInitialized);
        }
        Ok(())
    }

    /// Gets the admin address
    pub fn get_admin(&self) -> Result<Address, Error> {
        self.storage.get_admin()
    }

    /// Checks if the provided address is the admin
    pub fn is_admin(&self, address: &Address) -> Result<bool, Error> {
        let admin = self.get_admin()?;
        Ok(&admin == address)
    }

    /// Checks if the current invoker is the admin
    pub fn is_invoker_admin(&self) -> Result<bool, Error> {
        let invoker = self.env.invoker();
        self.is_admin(&invoker)
    }

    /// Requires that the invoker is the admin
    pub fn require_admin(&self) -> Result<(), Error> {
        if !self.is_invoker_admin()? {
            return Err(Error::AdminOnly);
        }
        Ok(())
    }

    /// Requires that the invoker is the specified authority
    pub fn require_authority(&self, authority_id: &Address) -> Result<(), Error> {
        // Require authentication from the authority address
        authority_id.require_auth();

        // Also verify the invoker is this authority
        let invoker = self.env.invoker();
        if &invoker != authority_id {
            return Err(Error::AuthorityOnly);
        }

        Ok(())
    }

    /// Requires that the invoker is the specified entity
    pub fn require_entity(&self, entity_id: &Address) -> Result<(), Error> {
        // Require authentication from the entity address
        entity_id.require_auth();

        // Also verify the invoker is this entity
        let invoker = self.env.invoker();
        if &invoker != entity_id {
            return Err(Error::EntityOnly);
        }

        Ok(())
    }

    /// Requires that the invoker is either the admin or the specified authority
    pub fn require_admin_or_authority(&self, authority_id: &Address) -> Result<(), Error> {
        let invoker = self.env.invoker();

        // Check if invoker is admin
        if self.is_admin(&invoker)? {
            return Ok(());
        }

        // Otherwise require authority authentication
        authority_id.require_auth();
        if &invoker != authority_id {
            return Err(Error::Unauthorized);
        }

        Ok(())
    }

    /// Verifies a digital signature using Ed25519
    pub fn verify_signature(
        &self,
        public_key: &BytesN<32>,
        message: &[u8],
        signature: &BytesN<64>
    ) -> Result<bool, Error> {
        // Convert types for signature verification
        let public_key_val = public_key.clone().into_val(self.env);
        let signature_val = signature.clone().into_val(self.env);
        let message_val = self.env.crypto().sha256(message);

        // Verify the signature
        let is_valid = self.env.crypto().ed25519_verify(
            &public_key_val,
            &message_val,
            &signature_val
        );

        Ok(is_valid)
    }
}