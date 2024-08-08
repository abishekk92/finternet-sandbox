use finternet_core::{
    primitives::core::{FinternetUID, PublicKey},
    primitives::user::User,
    smartcontract_interface::user_manager::UserManager,
};
use std::collections::HashMap;

pub struct SolanaUserManager;

impl UserManager for SolanaUserManager {
    fn create_user(&self, _user: User) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn update_user(&self, _user: User) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn delete_user(&self, _user_id: FinternetUID) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn rotate_key(&self, _user_id: FinternetUID, _new_key: PublicKey) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn accept_incoming_asset(
        &self,
        _user_id: FinternetUID,
        _asset_id: FinternetUID,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn update_asset_units(
        &self,
        _user_id: FinternetUID,
        _asset_id: FinternetUID,
        _new_units: u64,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }

    fn update_asset_config(
        &self,
        _user_id: FinternetUID,
        _asset_id: FinternetUID,
        _config: HashMap<String, String>,
    ) -> Result<String, String> {
        Ok("Success".to_string())
    }
}
