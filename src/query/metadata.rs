use crate::{
    error::ContractError,
    state::{
        models::MetadataResponse,
        storage::{SITE_DESCRIPTION, SITE_FAVICON, SITE_KEYWORDS, SITE_TITLE},
    },
};

use super::ReadonlyContext;

pub fn query_metadata(ctx: ReadonlyContext) -> Result<MetadataResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    let title = SITE_TITLE.load(deps.storage)?;
    let favicon = SITE_FAVICON.may_load(deps.storage)?;
    let keywords = SITE_KEYWORDS.load(deps.storage).unwrap_or_default();
    let description = SITE_DESCRIPTION.may_load(deps.storage)?;

    Ok(MetadataResponse {
        title,
        description,
        keywords,
        favicon,
    })
}
