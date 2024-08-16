use anchor_lang::prelude::*;

declare_id!("8Yrj4EMn2rWLmXLpnJWgWc1KXTLZ8DtneEfJ4cuPXr2N");

#[program]
pub mod user_manager {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, data: Vec<u8>) -> Result<()> {
        let log = EncryptedLog { data };
        emit_cpi!(log);
        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, data: Vec<u8>) -> Result<()> {
        let log = EncryptedLog { data };
        emit_cpi!(log);
        Ok(())
    }
    // fn delete_user(&self, _user_id: FinternetUID) -> Result<String, String> {
    //     Ok("Success".to_string())
    // }

    // fn rotate_key(&self, _user_id: FinternetUID, _new_key: PublicKey) -> Result<String, String> {
    //     Ok("Success".to_string())
    // }

    // fn accept_incoming_asset(
    //     &self,
    //     _user_id: FinternetUID,
    //     _asset_id: FinternetUID,
    // ) -> Result<String, String> {
    //     Ok("Success".to_string())
    // }

    // fn update_asset_units(
    //     &self,
    //     _user_id: FinternetUID,
    //     _asset_id: FinternetUID,
    //     _new_units: u64,
    // ) -> Result<String, String> {
    //     Ok("Success".to_string())
    // }

    // fn update_asset_config(
    //     &self,
    //     _user_id: FinternetUID,
    //     _asset_id: FinternetUID,
    //     _config: HashMap<String, String>,
    // ) -> Result<String, String> {
    //     Ok("Success".to_string())
    // }
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct CreateUser {}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct UpdateUser {}

#[event]
pub struct EncryptedLog {
    pub data: Vec<u8>,
}
