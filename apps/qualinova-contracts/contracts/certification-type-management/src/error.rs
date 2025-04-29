use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    // General errors
    AlreadyExists = 1,
    NotFound = 2,
    Unauthorized = 3,
    InvalidInput = 4,

    // Specific to certification types
    AlreadyDeprecated = 100,
    InvalidField = 101,
    InvalidStatus = 102,
    InvalidEvidence = 103,
    InvalidAuthority = 104,
}