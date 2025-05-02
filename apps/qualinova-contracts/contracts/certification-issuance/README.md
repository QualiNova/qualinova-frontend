# Certification Issuance Smart Contract

A Soroban smart contract for digital certificate issuance, verification, and management on the Stellar blockchain.

## Overview

This smart contract provides a comprehensive solution for creating, managing, and verifying digital certificates on the Stellar network. It enables educational institutions, professional organizations, and other certificate issuers to create tamper-proof digital certificates that can be easily verified by third parties.

## Features

- **Secure Certificate Issuance**: Issue digital certificates with cryptographic signatures
- **Certificate Transfer**: Transfer certificate ownership between addresses
- **Certificate Verification**: Verify the authenticity and validity of certificates
- **Certificate Revocation**: Revoke certificates that are no longer valid
- **Batch Operations**: Issue multiple certificates in a single transaction
- **Access Control**: Role-based permissions for admins and issuers
- **Pagination**: Efficient retrieval of certificates with pagination support
- **Event Emission**: Transparent tracking of all certificate operations

## Contract Structure

The contract is organized into several modules:

- **lib.rs**: Main contract implementation and data structures
- **access_control.rs**: Admin and issuer role management
- **issuance.rs**: Certificate issuance and batch operations
- **transfer.rs**: Certificate ownership transfer
- **verification.rs**: Certificate signature verification
- **query.rs**: Certificate retrieval and listing operations
- **test.rs**: Comprehensive test suite

## Data Structures

### Certificate

```rust
pub struct Certificate {
    pub id: CertificateId,
    pub owner: Address,
    pub issuer: Address,
    pub metadata: CertificateMetadata,
    pub issue_date: u64,
    pub expiration_date: Option<u64>,
    pub signature: Bytes,
    pub revoked: bool,
}
```

### CertificateMetadata

```rust
pub struct CertificateMetadata {
    pub title: String,
    pub description: String,
    pub achievement_type: String,
    pub additional_data: Map<String, BytesN<32>>,
}
```

## Contract Interface

### Initialization

- `initialize(admin: Address)`: Initialize the contract with an admin address

### Certificate Management

- `issue_certificate(owner: Address, metadata: CertificateMetadata, expiration_date: Option<u64>, signature: Bytes) -> CertificateId`
- `batch_issue_certificates(owners: Vec<Address>, metadatas: Vec<CertificateMetadata>, expiration_dates: Vec<Option<u64>>, signatures: Vec<Bytes>) -> Vec<CertificateId>`
- `revoke_certificate(certificate_id: CertificateId) -> bool`
- `transfer_certificate(certificate_id: CertificateId, new_owner: Address) -> bool`

### Certificate Queries

- `get_certificate(certificate_id: CertificateId) -> Certificate`
- `list_certificates_by_owner(owner: Address, start: u32, limit: u32) -> Vec<Certificate>`
- `list_certificates_by_issuer(issuer: Address, start: u32, limit: u32) -> Vec<Certificate>`
- `count_certificates_by_owner(owner: Address) -> u32`
- `count_certificates_by_issuer(issuer: Address) -> u32`
- `get_certificate_count() -> u32`

### Certificate Verification

- `verify_certificate_signature(certificate_id: CertificateId) -> bool`

### Access Control

- `add_issuer(issuer: Address) -> bool`
- `remove_issuer(issuer: Address) -> bool`
- `is_issuer(address: Address) -> bool`
- `get_issuers() -> Vec<Address>`
- `get_admin() -> Address`
- `transfer_admin(new_admin: Address) -> bool`

## Building and Testing

### Prerequisites

- Rust and Cargo
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- [Stellar Development Environment](https://developers.stellar.org/docs/fundamentals-and-concepts/stellar-development)

### Build

```bash
# Build the contract
make build
# or
stellar contract build
```

The compiled WASM file will be available at:
`target/wasm32-unknown-unknown/release/certification_issuance.wasm`

### Test

```bash
# Run all tests
make test
# or
cargo test
```

## Deployment

To deploy the contract to the Stellar network:

```bash
# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/certification_issuance.wasm \
  --source <your-secret-key> \
  --network testnet
```

## Security Considerations

- All certificate operations require proper authorization
- Admin role has full control over the contract
- Issuers can only issue and revoke certificates
- Certificate owners can only transfer their own certificates
- Certificate signatures should be verified using cryptographic methods

## Acknowledgments

- Soroban SDK Team
- Stellar Development Foundation