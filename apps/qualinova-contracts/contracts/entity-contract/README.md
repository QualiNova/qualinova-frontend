# Certifiable Entity Smart Contract

A Soroban smart contract for registering and managing organizations that can receive certifications on the Stellar blockchain.

## Overview

This smart contract provides a comprehensive solution for registering and managing entities (organizations) that can receive certifications on the Stellar network. It enables verification of entities and management of their certifications.

## Features

- **Entity Registration**: Register organizations with essential information
- **Entity Information Updates**: Modify existing entity details
- **Entity Verification**: Verify the status of an entity
- **Entity Deactivation**: Deactivate entities with reason tracking
- **Certification Tracking**: Track certifications associated with entities
- **Admin Management**: Role-based permissions for administrative functions

## Contract Structure

The contract is organized into several modules:

- **lib.rs**: Main contract implementation and data structures
- **admin.rs**: Admin role management
- **entity.rs**: Entity registration and management operations
- **types.rs**: Data structure definitions
- **test.rs**: Comprehensive test suite

## Data Structure

### Entity

```rust
pub struct Entity {
    pub entity_id: String,             // Unique identifier for the entity
    pub name: String,                  // Name of the organization
    pub public_key: Address,           // Stellar public key of the entity
    pub registration_date: u64,        // Timestamp of registration
    pub industry_sector: String,       // The industry sector (e.g., "Automotive")
    pub location: String,              // Location (e.g., "San Francisco, CA")
    pub contact_info: String,          // Contact information (e.g., email)
    pub status: EntityStatus,          // Active, Inactive
    pub certifications: Vec<String>,   // List of certification IDs
}
```

## Contract Interface

### Initialization

- `initialize(admin: Address)`: Initialize the contract with an admin address

### Entity Management

- `register_entity(name: String, industry_sector: String, location: String, contact_info: String, public_key: Address) -> String`: Register a new entity
- `update_entity_info(entity_id: Address, field: String, value: String)`: Update specific entity fields
- `verify_entity(entity_id: Address) -> bool`: Check if an entity is active
- `deactivate_entity(entity_id: Address, reason: String)`: Deactivate an entity
- `list_entity_certifications(entity_id: Address) -> Vec<String>`: List all certifications for an entity
- `get_entity(entity_id: Address) -> Entity`: Get detailed entity information

### Admin Management

- `get_admin() -> Address`: Get the current admin address
- `transfer_admin(current_admin: Address, new_admin: Address)`: Transfer admin role to a new address

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
`target/wasm32-unknown-unknown/release/entity_contract.wasm`

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
  --wasm target/wasm32-unknown-unknown/release/entity_contract.wasm \
  --source <your-secret-key> \
  --network testnet
```

## Security Considerations

- All entity operations require proper authorization
- Admin role has full control over the contract
- Entity owners can only update their own entity information
- Entity status verification ensures only active entities are considered valid