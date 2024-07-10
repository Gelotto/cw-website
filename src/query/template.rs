use cosmwasm_std::{Order, StdResult};

use crate::{
    error::ContractError,
    msg::TemplatesResponse,
    state::{
        models::{TemplateInfo, TemplateResponse},
        storage::{ROUTE_KEYWORDS, ROUTE_SCRIPT_NAMES, ROUTE_STYLE_NAMES, ROUTE_TEMPLATES},
    },
};

use super::ReadonlyContext;

pub fn query_template(
    ctx: ReadonlyContext,
    path: String,
) -> Result<TemplateResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    // Ensure all path is absolute
    let mut path = path;
    if !path.starts_with("/") {
        path = format!("/{}", path);
    }

    let source = ROUTE_TEMPLATES.load(deps.storage, &path)?;
    let keywords = ROUTE_KEYWORDS.load(deps.storage, &path).unwrap_or_default();
    let scripts = ROUTE_SCRIPT_NAMES.load(deps.storage, &path).unwrap_or_default();
    let styles = ROUTE_STYLE_NAMES.load(deps.storage, &path).unwrap_or_default();

    Ok(TemplateResponse {
        path,
        keywords,
        scripts,
        styles,
        source,
    })
}
