/// User Manager Smart Contract Interface/Traits
use crate::{
    primitives::core::{FinternetUID, PublicKey},
    primitives::user::User,
};

// User Trait
pub trait UserManager {
    fn create_user(&self, user: User) -> Result<String, String>;
    fn update_user(&self, user: User) -> Result<String, String>;
    fn delete_user(&self, user_id: FinternetUID) -> Result<String, String>;
    fn rotate_key(&self, user_id: FinternetUID, new_key: PublicKey) -> Result<String, String>;
    fn accept_incoming_asset(
        &self,
        user_id: FinternetUID,
        asset_id: FinternetUID,
    ) -> Result<String, String>;
}
