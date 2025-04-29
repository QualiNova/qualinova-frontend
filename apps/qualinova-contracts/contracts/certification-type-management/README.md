# Certification Type Management Contract

## Overview

The Certification Type Management Contract is a Soroban smart contract built for the Stellar blockchain. It provides functionality to define, manage, and track different types of certifications available in the system.

This contract is a core component of the Qualinova certification management platform, allowing authorities to register certification types and specify their requirements.

## Features

- **Register Certification Types**: Create new certification types with industry scope and validity period
- **Update Certification Types**: Modify existing certification type details
- **Deprecate Certification Types**: Mark certification types as deprecated with a reason
- **Manage Required Evidence**: Define and update evidence required for each certification type
- **Assign Authorities**: Connect certification authorities to specific certification types
- **List and Query**: Access comprehensive lists and detailed information about certification types

## Data Structure

Each certification type contains the following information:

```
{
  "cert_type_id": "UNIQUE_TYPE_ID",
  "name": "ISO_9001",
  "version": "2015",
  "description": "Quality Management System standard",
  "industry_scope": ["Manufacturing", "Services", "Healthcare"],
  "validity_period": "3 years",
  "required_evidence": ["Audit Report", "Conformity Statement"],
  "verification_requirements": "Annual surveillance audit",
  "status": "ACTIVE",
  "authorities": ["AUTH_001", "AUTH_002"],
  "deprecation_reason": null
}
```

## Key Functions

| Function | Description |
|----------|-------------|
| `register_certification_type` | Creates a new certification type with a unique ID |
| `update_certification_type` | Updates specific fields for an existing certification type |
| `deprecate_certification_type` | Marks a certification type as deprecated |
| `list_all_certification_types` | Returns a list of all certification types |
| `get_certification_type_details` | Gets detailed information about a specific certification type |
| `set_required_evidence` | Updates the evidence required for a certification type |
| `assign_authority_to_cert_type` | Assigns an authority to a certification type |

## Error Handling

The contract includes robust error handling for various scenarios:

- Already exists (when trying to register a duplicate)
- Not found (when querying a non-existent certification type)
- Already deprecated (when trying to modify a deprecated certification type)
- Invalid field (when trying to update a field that doesn't exist)
- Other specialized errors for specific operations

## Development

### Prerequisites

- Rust toolchain
- Soroban CLI
- Stellar network access (testnet or local development network)

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test
```

### Deploy

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/certification_type_management.wasm \
  --source <source-account> \
  --network <network>
```

## Integration with Other Contracts

This contract is designed to work with other contracts in the Qualinova ecosystem, including:

- Certification Authority Contract
- Certification Verification Contract
- Certification Issuance Contract

The Qualinova development team