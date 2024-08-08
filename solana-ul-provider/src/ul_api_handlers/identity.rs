use finternet_core::primitives::core::PublicKey;

/// Create Identity
pub fn create(_pubkey: PublicKey) -> Result<String, String> {
    Ok("Identity created".to_string())
}

/// RotateKey
pub fn rotate_key(_id: String, _new_key: PublicKey) -> Result<String, String> {
    Ok("Key rotated".to_string())
}

/// Close Identity
pub fn close(_id: String) -> Result<String, String> {
    Ok("Identity closed".to_string())
}

// TODO: Implement get_asset_units
// Holding off to come back later with concrete types

/// Attach Credential
pub fn attach_credential(_id: String, _cred: String) -> Result<String, String> {
    Ok("Credential attached".to_string())
}
