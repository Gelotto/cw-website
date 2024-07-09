use cosmwasm_std::{Binary, StdError};

use crate::{
    error::ContractError,
    state::{models::Asset, storage::ROUTE_ASSETS},
};

use super::ReadonlyContext;

pub fn query_asset(
    ctx: ReadonlyContext,
    name: String,
) -> Result<Asset, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    ROUTE_ASSETS.load(deps.storage, &name).map_err(|e| {
        ContractError::Std(StdError::generic_err(format!(
            "failed to load asset {}: {}",
            name,
            e.to_string()
        )))
    })
}
