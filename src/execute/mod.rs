pub mod set_config;
pub mod set_metadata;
pub mod upsert_asset;
pub mod upsert_template;

use cosmwasm_std::{DepsMut, Env, MessageInfo};

pub struct Context<'a> {
    pub deps: DepsMut<'a>,
    pub env: Env,
    pub info: MessageInfo,
}
