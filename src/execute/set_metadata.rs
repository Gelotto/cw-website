use crate::{
    error::ContractError,
    state::{
        models::Link,
        storage::{SITE_DESCRIPTION, SITE_FAVICON, SITE_KEYWORDS, SITE_TITLE},
    },
};
use cosmwasm_std::{attr, Response};

use super::Context;

pub fn exec_set_metadata(
    ctx: Context,
    title: String,
    description: Option<String>,
    keywords: Option<Vec<String>>,
    favicon: Option<Link>,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    SITE_TITLE.save(deps.storage, &title)?;

    if let Some(description) = description {
        SITE_DESCRIPTION.save(deps.storage, &description)?;
    } else {
        SITE_DESCRIPTION.remove(deps.storage)
    }

    if let Some(keywords) = keywords {
        SITE_KEYWORDS.save(deps.storage, &keywords)?;
    } else {
        SITE_KEYWORDS.remove(deps.storage)
    }

    if let Some(favicon) = favicon {
        SITE_FAVICON.save(deps.storage, &favicon)?;
    } else {
        SITE_FAVICON.remove(deps.storage)
    }

    Ok(Response::new().add_attributes(vec![attr("action", "set_metadata")]))
}
