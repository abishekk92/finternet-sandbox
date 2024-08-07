/// Token Manager Smart Contract Interface/Traits
use crate::{primitives::asset::AssetStatus, primitives::core::FinternetUID};
use std::collections::HashMap;

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
    fn approve_transfer(
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
