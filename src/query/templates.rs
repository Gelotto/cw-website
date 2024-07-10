use cosmwasm_std::{Order, StdResult};

use crate::{
    error::ContractError,
    msg::TemplatesResponse,
    state::{
        models::TemplateInfo,
        storage::{ROUTE_KEYWORDS, ROUTE_SCRIPT_NAMES, ROUTE_STYLE_NAMES, ROUTE_TEMPLATES},
    },
};

use super::ReadonlyContext;

pub fn query_templates(ctx: ReadonlyContext) -> Result<TemplatesResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let mut infos: Vec<TemplateInfo> = Vec::with_capacity(8);

    for result in ROUTE_TEMPLATES
        .keys(deps.storage, None, None, Order::Ascending)
        .collect::<Vec<StdResult<_>>>()
    {
        let path = result?;
        let keywords = ROUTE_KEYWORDS.load(deps.storage, &path).unwrap_or_default();
        let scripts = ROUTE_SCRIPT_NAMES.load(deps.storage, &path).unwrap_or_default();
        let styles = ROUTE_STYLE_NAMES.load(deps.storage, &path).unwrap_or_default();

        infos.push(TemplateInfo {
            path,
            keywords,
            scripts,
            styles,
        })
    }

    Ok(TemplatesResponse(infos))
}
