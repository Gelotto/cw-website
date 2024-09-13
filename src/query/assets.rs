use crate::{
    error::ContractError,
    msg::TemplateAssetsResponse,
    state::{
        models::AssetInfo,
        storage::{ROUTE_SCRIPT_NAMES, ROUTE_STYLE_NAMES, SCRIPT_ASSETS, STYLE_ASSETS},
    },
};

use super::ReadonlyContext;

pub fn query_assets(
    ctx: ReadonlyContext,
    path: String,
) -> Result<TemplateAssetsResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let mut scripts: Vec<AssetInfo> = Vec::with_capacity(2);
    let mut styles: Vec<AssetInfo> = Vec::with_capacity(2);

    for name in ROUTE_SCRIPT_NAMES.load(deps.storage, &path).unwrap_or_default() {
        let asset = SCRIPT_ASSETS.load(deps.storage, &name)?;
        scripts.push(AssetInfo {
            mime_type: asset.mime_type,
            name: asset.name,
            data: asset.data,
        })
    }

    for name in ROUTE_STYLE_NAMES.load(deps.storage, &path).unwrap_or_default() {
        let asset = STYLE_ASSETS.load(deps.storage, &name)?;
        styles.push(AssetInfo {
            mime_type: asset.mime_type,
            name: asset.name,
            data: asset.data,
        })
    }

    Ok(TemplateAssetsResponse { scripts, styles })
}
