use crate::{
    error::ContractError,
    state::{models::Asset, storage::ROUTE_ASSETS},
};
use cosmwasm_std::{attr, Binary, Response};

use super::Context;

pub fn exec_upsert_asset(
    ctx: Context,
    name: String,
    mime_type: String,
    data: Binary,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;

    ROUTE_ASSETS.save(
        deps.storage,
        &name,
        &Asset {
            name: name.to_owned(),
            mime_type,
            data,
        },
    )?;

    Ok(Response::new().add_attributes(vec![attr("action", "upsert_asset")]))
}
