use crate::{
    error::ContractError,
    state::storage::{ROUTE_SCRIPT_NAMES, ROUTE_STYLE_NAMES, ROUTE_TEMPLATES},
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_upsert_template(
    ctx: Context,
    path: String,
    template: String,
    scripts: Option<Vec<String>>,
    styles: Option<Vec<String>>,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;

    ROUTE_TEMPLATES.save(deps.storage, &path, &template)?;

    if let Some(scripts) = scripts {
        ROUTE_SCRIPT_NAMES.save(deps.storage, &path, &scripts)?;
    }

    if let Some(styles) = styles {
        ROUTE_STYLE_NAMES.save(deps.storage, &path, &styles)?;
    }

    Ok(Response::new().add_attributes(vec![attr("action", "upsert_template")]))
}
