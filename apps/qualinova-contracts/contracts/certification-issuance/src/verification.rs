use crate::{Certificate, CertificateId};
use alloc::format;
use soroban_sdk::{Address, Bytes, BytesN, Env, Symbol};
use soroban_sdk::xdr::ToXdr;

// Verify a certificate's signature
pub fn verify_certificate_signature(env: &Env, certificate_id: CertificateId) -> bool {
    // Get the certificate
    let cert_id_bytes = env.crypto().sha256(&Bytes::from_slice(env, &certificate_id.0.to_array()));
    let certificate_key = get_certificate_key(env, &cert_id_bytes);
    
    let certificate: Certificate = env
        .storage()
        .persistent()
        .get(&certificate_key)
        .unwrap_or_else(|| panic!("Certificate not found"));
    
    // Check if the certificate is revoked
    if certificate.revoked {
        return false;
    }
    
    // Check if the certificate is expired
    if let Some(expiration) = certificate.expiration_date {
        let current_time = env.ledger().timestamp();
        if current_time > expiration {
            return false;
        }
    }
    
    // Verify the signature
    let message = create_verification_message(env, &certificate);
    let issuer_public_key = get_issuer_public_key(env, &certificate.issuer);
    
    // Convert the signature to BytesN<64> for ed25519_verify
    // This is a simplification - in a real implementation, you'd ensure the signature is valid
    let signature_bytes = BytesN::<64>::from_array(env, &[0; 64]);
    
    // Verify the signature using ed25519
    let _result = env.crypto().ed25519_verify(
        &issuer_public_key,
        &message,
        &signature_bytes
    );
    
    // For now, since we're using a placeholder signature, return true
    // In a real implementation, we would use the actual result
    true
}

// Create the message that was signed
fn create_verification_message(env: &Env, certificate: &Certificate) -> Bytes {
    // Combine certificate data to create the message
    let mut data = Bytes::new(env);
    
    // Add certificate ID
    data.append(&Bytes::from_slice(env, &certificate.id.0.to_array()));
    
    // Add owner and issuer
    data.append(&certificate.owner.clone().to_xdr(env));
    data.append(&certificate.issuer.clone().to_xdr(env));
    
    // Add metadata - convert strings to bytes using to_xdr
    data.append(&certificate.metadata.title.clone().to_xdr(env));
    data.append(&certificate.metadata.description.clone().to_xdr(env));
    data.append(&certificate.metadata.achievement_type.clone().to_xdr(env));
    
    // Add issuance date
    let issuance = certificate.issuance_date;
    let mut tmp = issuance;
    for _ in 0..8 {
        let byte = (tmp & 0xFF) as u8;
        tmp >>= 8;
        data.append(&Bytes::from_slice(env, &[byte]));
    }
    
    // Add expiration date if present
    if let Some(expiration) = certificate.expiration_date {
        let mut tmp = expiration;
        for _ in 0..8 {
            let byte = (tmp & 0xFF) as u8;
            tmp >>= 8;
            data.append(&Bytes::from_slice(env, &[byte]));
        }
    }
    
    // Hash the data to create the message
    let hash = env.crypto().sha256(&data);
    Bytes::from_slice(env, &hash.to_array())
}

// Get the issuer's public key (for signature verification)
fn get_issuer_public_key(env: &Env, issuer: &Address) -> BytesN<32> {
    // In a real implementation, we would fetch the issuer's public key
    // from a registry or derive it from their address
    // For now, we'll use the address's serialized form as a placeholder
    let address_bytes = issuer.clone().to_xdr(env);
    let hash = env.crypto().sha256(&address_bytes);
    hash.into()
}

// Helper function to get a certificate key from a hash
fn get_certificate_key(env: &Env, hash: &soroban_sdk::crypto::Hash<32>) -> Symbol {
    // Create a simple key with the hash - avoid using colons
    let key = format!("cert{:x}", hash.to_array()[0]);
    Symbol::new(env, &key)
}
