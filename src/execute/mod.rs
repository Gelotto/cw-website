pub mod set_config;
pub mod set_metadata;
pub mod upsert_asset;
pub mod upsert_template;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

use crate::error::ContractError;
use crate::state::storage::ADMIN;

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}

/// Verify that the sender is the contract admin
pub fn assert_admin(ctx: &Context) -> Result<(), ContractError> {
    let admin = ADMIN.load(ctx.deps.storage)?;
    if ctx.info.sender != admin {
        return Err(ContractError::NotAuthorized {
            reason: format!("Only admin {} can execute", admin),
        });
    }
    Ok(())
}
