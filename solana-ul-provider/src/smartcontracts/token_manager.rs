use finternet_core::smartcontract_interface::token_manager::TokenManager;
use std::collections::HashMap;

pub struct ABCTokenManager;

impl TokenManager for ABCTokenManager {
    fn tokenize(
        &self,
        _asset_id: finternet_core::primitives::core::FinternetUID,
        _user_id: finternet_core::primitives::core::FinternetUID,
        _units: u64,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn detokenize(
        &self,
        _token_id: finternet_core::primitives::core::FinternetUID,
        _user_id: finternet_core::primitives::core::FinternetUID,
        _units: u64,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn approve_transfer(
        &self,
        _asset_id: finternet_core::primitives::core::FinternetUID,
        _from_user_id: finternet_core::primitives::core::FinternetUID,
        _to_user_id: finternet_core::primitives::core::FinternetUID,
        _units: u64,
    ) -> Result<String, String> {
        Ok("Approved".to_string())
    }

    fn update_asset_status(
        &self,
        _asset_id: finternet_core::primitives::core::FinternetUID,
        _status: finternet_core::primitives::asset::AssetStatus,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn update_asset_metadata(
        &self,
        _asset_id: finternet_core::primitives::core::FinternetUID,
        _metadata: HashMap<String, String>,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }
}
