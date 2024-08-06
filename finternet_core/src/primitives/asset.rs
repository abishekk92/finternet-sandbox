/// Types and traits for working with assets.
use crate::primitives::core::FinternetUID;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Asset Struct
/// Asset struct to store the asset details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Asset ID
    pub id: FinternetUID,
    /// Asset Name
    pub name: String,
    /// Asset Description
    pub description: String,
    /// Asset Token Manager
    pub token_manager: FinternetUID, // Give the token manager a proper type
    /// Created At
    pub created_at: u64,
    /// Modified At
    pub modified_at: u64,
    /// Asset Type
    pub asset_type: AssetType,
    /// Asset Status
    pub status: AssetStatus,
    /// Holder
    pub holder: FinternetUID,
    pub asset_metadata: HashMap<String, String>, // Asset Metadata
}

// Asset Type
/// Asset Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetType {
    /// Fungible Asset
    Fungible,
    /// Non-Fungible Asset
    NonFungible,
}

// Asset Status
/// Asset Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssetStatus {
    /// Locked Asset
    Locked,
    /// Unlocked Asset
    Unlocked,
}
