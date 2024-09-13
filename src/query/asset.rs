use cosmwasm_std::StdError;

use crate::{
    error::ContractError,
    state::{
        models::{Asset, AssetType},
        storage::{SCRIPT_ASSETS, STYLE_ASSETS},
    },
};

use super::ReadonlyContext;

pub fn query_asset(
    ctx: ReadonlyContext,
    asset_type: AssetType,
    name: String,
) -> Result<Asset, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let map = match asset_type {
        AssetType::Script => SCRIPT_ASSETS,
        AssetType::Style => STYLE_ASSETS,
    };
    map.load(deps.storage, &name).map_err(|e| {
        ContractError::Std(StdError::generic_err(format!(
            "failed to load asset {}: {}",
            name,
            e.to_string()
        )))
    })
}
