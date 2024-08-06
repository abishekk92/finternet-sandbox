/// Types and traits for working with users.
use crate::primitives::core::{FinternetChainID, FinternetUID, PublicKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    /// User Asset Config
    pub asset_config: HashMap<FinternetUID, HashMap<String, String>>, // Asset specific config like
    // whitelisted senders, spend limits etc
    /// Incoming Asset Ledger
    pub incoming_asset_ledger: HashMap<FinternetUID, HashMap<FinternetUID, u64>>, // Incoming Asset Ledger for the user (Asset ID, Sender, Units)
    /// Outgoing Asset Ledger
    pub outgoing_asset_ledger: HashMap<FinternetUID, HashMap<FinternetUID, u64>>, // Incoming Asset Ledger for the user (Asset ID, Sender, Units)
    /// User Asset Ledger
    pub asset_ledger: HashMap<FinternetUID, u64>, // Asset Ledger for the user (Asset ID, Units)
}
