pub mod asset;
pub mod assets;
pub mod config;
pub mod metadata;
pub mod render;
pub mod template;
pub mod templates;

use cosmwasm_std::{Deps, Env};

pub struct ReadonlyContext<'a> {
    pub deps: Deps<'a>,
    pub env: Env,
}
