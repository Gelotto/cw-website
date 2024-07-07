use crate::{error::ContractError, state::storage::TEMPLATES};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_upsert_template(
    ctx: Context,
    path: String,
    template: String,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    TEMPLATES.save(deps.storage, &path, &template)?;
    Ok(Response::new().add_attributes(vec![attr("action", "upsert_template")]))
}
