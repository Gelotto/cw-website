use crate::{
    error::ContractError,
    state::{
        models::{Asset, AssetType},
        storage::{SCRIPT_ASSETS, STYLE_ASSETS},
    },
};
use cosmwasm_std::{attr, Binary, Response};

use super::Context;

pub fn exec_upsert_asset(
    ctx: Context,
    name: String,
    asset_type: AssetType,
    mime_type: String,
    data: Binary,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;

    let map = match asset_type {
        AssetType::Script => SCRIPT_ASSETS,
        AssetType::Style => STYLE_ASSETS,
    };

    map.save(
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
