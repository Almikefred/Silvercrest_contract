//! On-chain error and version invariants.

use silvercrest_contract::{RegistryError, VERSION};

#[test]
fn version_is_three() {
    assert_eq!(VERSION, 3);
}

#[test]
fn zero_price_error_code() {
    assert_eq!(RegistryError::ZeroPrice as u32, 6);
}

#[test]
fn paused_error_code() {
    assert_eq!(RegistryError::Paused as u32, 7);
}

#[test]
fn already_initialized_error_code() {
    assert_eq!(RegistryError::AlreadyInitialized as u32, 9);
}
