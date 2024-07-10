use cosmwasm_std::Order;

use crate::{
    error::ContractError,
    state::{models::AssetInfo, storage::ASSETS},
};

use super::ReadonlyContext;

pub fn query_assets(ctx: ReadonlyContext) -> Result<Vec<AssetInfo>, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(ASSETS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|r| {
            let (_, asset) = r.unwrap();
            AssetInfo {
                mime_type: asset.mime_type,
                name: asset.name,
            }
        })
        .collect())
}
