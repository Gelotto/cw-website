use crate::{
    error::ContractError,
    state::{models::Config, storage::CONFIG},
};
use cosmwasm_std::{attr, Response};

use super::{assert_admin, Context};

pub fn exec_set_config(
    ctx: Context,
    config: Config,
) -> Result<Response, ContractError> {
    assert_admin(&ctx)?;
    let Context { deps, .. } = ctx;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![attr("action", "set_config")]))
}
