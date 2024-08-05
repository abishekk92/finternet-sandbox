use serde::{Deserialize, Deserializer, Serialize};
use ssi_caips::caip2::ChainId;
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;

/// Fundamental Identifier for anything on the Finternet
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinternetUID([u8; 32]);

/// Public Key or Signing Key for the User
// __NOTE__
// This should be generalized to support multiple key types or eliptic curves
// The current implementation is for the sake of simplicity and to get the ball rolling
// It doesn't  explicitly say anything about the key type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKey([u8; 32]);

/// Type wrappe for ChainId
#[derive(Debug, Clone)]
pub struct FinternetChainID(ChainId);

// impl all the necessary traits for FinternetChainID
impl Deref for FinternetChainID {
    type Target = ChainId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for FinternetChainID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Value::String(self.0.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FinternetChainID {
    fn deserialize<D>(deserializer: D) -> Result<FinternetChainID, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let chain_id_str = String::deserialize(deserializer)?;
        let chain_id = ChainId::from_str(&chain_id_str).map_err(serde::de::Error::custom)?;
        Ok(FinternetChainID(chain_id))
    }
}

// User Struct
/// User struct to store the user details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: FinternetUID,
    /// User Public Key
    pub public_key: PublicKey,
    /// Created At
    pub created_at: u64,
    /// Modified At
    pub modified_at: u64,
    /// User Metadata
    pub user_metadata: HashMap<String, String>, // User Metadata
    /// User Preferred Chain ID
    pub preferred_chain: FinternetChainID,
    /// User Asset Ledger
    pub asset_ledger: HashMap<FinternetUID, u64>, // Asset Ledger for the user (Asset ID, Units)
    /// User Asset Config
    pub asset_config: HashMap<FinternetUID, HashMap<String, String>>, // Asset specific config like
                                                                      // whitelisted senders, spend limits etc
}

/// Trait for loading the data into memory
// __NOTE__: Firm up the return type, impl Deserializer is lazy
pub trait Loader {
    fn load(&self, id: FinternetUID) -> Result<impl Deserializer, String>;
}

// User Trait
pub trait UserManager {
    fn create_user(&self, user: User) -> Result<FinternetUID, String>;
    fn update_user(&self, user: User) -> Result<FinternetUID, String>;
    fn delete_user(&self, user_id: FinternetUID) -> Result<FinternetUID, String>;
    fn rotate_key(&self, user_id: FinternetUID, new_key: PublicKey)
        -> Result<FinternetUID, String>;
    fn accept_incoming_asset(
        &self,
        user_id: FinternetUID,
        asset_id: FinternetUID,
    ) -> Result<FinternetUID, String>;
    fn update_asset_units(
        &self,
        user_id: FinternetUID,
        asset_id: FinternetUID,
        new_units: u64,
    ) -> Result<FinternetUID, String>;
    fn update_asset_config(
        &self,
        user_id: FinternetUID,
        asset_id: FinternetUID,
        config: HashMap<String, String>,
    ) -> Result<FinternetUID, String>;
}

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
    pub holders: Vec<FinternetUID>, // Give the holders a proper type. Holder is a user
    // NOTE: This could be problematic for multi holders
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

// Token Manager Trait
pub trait TokenManager {
    fn tokenize(
        &self,
        asset_id: FinternetUID,
        user_id: FinternetUID,
        units: u64,
    ) -> Result<FinternetUID, String>;
    fn detokenize(
        &self,
        asset_id: FinternetUID,
        user_id: FinternetUID,
        units: u64,
    ) -> Result<FinternetUID, String>;
    fn transfer(
        &self,
        asset_id: FinternetUID,
        from_user_id: FinternetUID,
        to_user_id: FinternetUID,
        units: u64,
    ) -> Result<FinternetUID, String>;
    fn authorize_transfer(
        &self,
        asset_id: FinternetUID,
        from_user_id: FinternetUID,
        to_user_id: FinternetUID,
        units: u64,
    ) -> Result<bool, String>;
    fn update_asset_status(
        &self,
        asset_id: FinternetUID,
        status: AssetStatus,
    ) -> Result<FinternetUID, String>;
    fn update_asset_metadata(
        &self,
        asset_id: FinternetUID,
        metadata: HashMap<String, String>,
    ) -> Result<FinternetUID, String>;
}
