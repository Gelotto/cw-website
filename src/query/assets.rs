use cosmwasm_std::Order;

use crate::{
    error::ContractError,
    state::{models::Asset, storage::ASSETS},
};

use super::ReadonlyContext;

pub fn query_assets(ctx: ReadonlyContext) -> Result<Vec<Asset>, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(ASSETS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|r| {
            let (_, asset) = r.unwrap();
            asset
        })
        .collect())
}
