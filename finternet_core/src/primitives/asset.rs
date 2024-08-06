/// Types and traits for working with assets.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::core::{FinternetChainID, FinternetUID};

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
    /// Asset Metadata
    // __NOTE__: Important to provide a provision for metadata
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

// // Asset Status
// /// Asset Status
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum AssetStatus {
//     /// Locked Asset
//     Locked,
//     /// Unlocked Asset
//     Unlocked,
// }

// Holder and there share in the asset
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolderConfig {
    pub holder_id: FinternetUID,
    pub ownership_basis_points: u16,
}

// basic actions that are possible on the asset for real world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorityType {
    Delegate,
    Lock,
    // ... More can come here
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorityConfig {
    pub authority_type: AuthorityType,
    pub authority_id: Option<FinternetUID>, // this can be a bank [ Token Manager ] or someone depending how its use
}
// AssetInstance Struct
/// Instance of the Asset struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetInstance {
    /// Asset ID
    pub asset_id: FinternetUID,
    /// Holder of the Asset
    pub holders: Vec<HolderConfig>,
    /// Units of the Asset
    pub units: u64,
    /// FinternetChainID of the Asset as per user preference
    // __NOTE__
    // Care should be taken to ensure that the chain_id is the same as the user's preferred chain_id
    // Very easy to mess up. We will deal with this complexity later
    // The user might prefer to hold different assets on different chains
    pub chain_id: FinternetChainID,
    /// Asset Token Manager
    pub token_manager: FinternetUID,
    /// Created At
    pub created_at: u64,
    /// Modified At
    pub modified_at: u64,
    /// Asset Status
    // pub status: AssetStatus,
    // Config of the assest helps with access control
    pub asset_authority_config: Vec<AuthorityConfig>,
}
