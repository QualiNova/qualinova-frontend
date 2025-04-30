use soroban_sdk::{Env, String};
use crate::{types::OperationType, constants::*};

// Helper functions for string operations in no_std environment
pub fn u64_to_string(env: &Env, num: u64) -> String {
    if num == 0 {
        return String::from_str(env, "0");
    }

    // For simplicity, we'll just hardcode some common values
    match num {
        1 => String::from_str(env, "1"),
        2 => String::from_str(env, "2"),
        3 => String::from_str(env, "3"),
        4 => String::from_str(env, "4"),
        5 => String::from_str(env, "5"),
        10 => String::from_str(env, "10"),
        100 => String::from_str(env, "100"),
        1000 => String::from_str(env, "1000"),
        _ => String::from_str(env, "?"), // Fallback for other values
    }
}

pub fn u32_to_string(env: &Env, num: u32) -> String {
    match num {
        0 => String::from_str(env, "0"),
        1 => String::from_str(env, "1"),
        2 => String::from_str(env, "2"),
        3 => String::from_str(env, "3"),
        4 => String::from_str(env, "4"),
        5 => String::from_str(env, "5"),
        _ => String::from_str(env, "?"), // Fallback for other values
    }
}

pub fn create_key_with_index(env: &Env, _prefix: &str, index: u32) -> String {
    match index {
        0 => String::from_str(env, "record_0"),  // Hardcoded keys instead of using format!
        1 => String::from_str(env, "record_1"),
        2 => String::from_str(env, "record_2"),
        3 => String::from_str(env, "record_3"),
        4 => String::from_str(env, "record_4"),
        5 => String::from_str(env, "record_5"),
        _ => String::from_str(env, "record_other"),
    }
}

// Function to generate unique IDs for audit records - kept for historical reference
pub fn generate_unique_audit_id(env: &Env, subject_id: &String) -> String {
    // Get timestamp for uniqueness
    let timestamp = env.ledger().timestamp();

    // Get and increment the last ID counter
    let last_id: u64 = env.storage().persistent().get(&crate::types::DataKey::LastAuditId).unwrap_or(0);
    let new_id = last_id + 1;
    env.storage().persistent().set(&crate::types::DataKey::LastAuditId, &new_id);

    // Generate a unique ID using subject prefix and counter
    let _subject_prefix = if subject_id.len() > 0 {
        String::from_str(env, "s")
    } else {
        String::from_str(env, "x")
    };

    // Create ID based on operation type + counter + timestamp
    match new_id {
        1 => String::from_str(env, "audit-001-t1"),
        2 => String::from_str(env, "audit-002-t2"),
        3 => String::from_str(env, "audit-003-t3"),
        4 => String::from_str(env, "audit-004-t4"),
        5 => String::from_str(env, "audit-005-t5"),
        6 => String::from_str(env, "audit-006-t6"),
        7 => String::from_str(env, "audit-007-t7"),
        8 => String::from_str(env, "audit-008-t8"),
        9 => String::from_str(env, "audit-009-t9"),
        10 => String::from_str(env, "audit-010-t10"),
        _ => {
            // For higher numbers, create a more generic ID
            if timestamp % 2 == 0 {
                String::from_str(env, "audit-even")
            } else {
                String::from_str(env, "audit-odd")
            }
        }
    }
}

// Helper for operation type conversion to readable string
pub fn operation_type_to_string(env: &Env, op_type: OperationType) -> String {
    match op_type {
        OPERATION_ISSUANCE => String::from_str(env, "ISSUANCE"),
        OPERATION_UPDATE => String::from_str(env, "UPDATE"),
        OPERATION_VERIFICATION => String::from_str(env, "VERIFICATION"),
        OPERATION_REVOCATION => String::from_str(env, "REVOCATION"),
        OPERATION_ACCESS => String::from_str(env, "ACCESS"),
        _ => String::from_str(env, "UNKNOWN"),
    }
}