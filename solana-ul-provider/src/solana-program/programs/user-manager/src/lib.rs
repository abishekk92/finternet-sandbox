use anchor_lang::prelude::*;

declare_id!("CmFuqQTLs2nQof5uaktJn1a6k2VdbGmZPfrJufB2Vm3F");

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

    pub fn delete_user(ctx: Context<DeleteUser>, data: Vec<u8>) -> Result<()> {
        let log = EncryptedLog { data };
        emit_cpi!(log);
        Ok(())
    }

    pub fn rotate_key(ctx: Context<RotateKey>, data: Vec<u8>) -> Result<()> {
        let log = EncryptedLog { data };
        emit_cpi!(log);
        Ok(())
    }

    pub fn accept_incoming_asset(ctx: Context<AcceptIncomingAsset>, data: Vec<u8>) -> Result<()> {
        let log = EncryptedLog { data };
        emit_cpi!(log);
        Ok(())
    }
}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct CreateUser {}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct UpdateUser {}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct DeleteUser {}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct RotateKey {}

#[event_cpi]
#[derive(Accounts)]
#[instruction(data: Vec<u8>)]
pub struct AcceptIncomingAsset {}

#[event]
pub struct EncryptedLog {
    pub data: Vec<u8>,
}
