pub mod models;
pub mod storage;

use cosmwasm_std::Response;
use storage::{ADMIN, CONFIG, SITE_DESCRIPTION, SITE_FAVICON, SITE_KEYWORDS, SITE_TITLE};

use crate::{error::ContractError, execute::Context, msg::InstantiateMsg};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, info, .. } = ctx;

    // Set the instantiator as admin
    ADMIN.save(deps.storage, &info.sender)?;

    // Save required fields
    SITE_TITLE.save(deps.storage, &msg.title)?;
    CONFIG.save(deps.storage, &msg.config)?;

    // Save optional metadata fields if provided
    if let Some(description) = &msg.description {
        SITE_DESCRIPTION.save(deps.storage, description)?;
    }
    if let Some(keywords) = &msg.keywords {
        SITE_KEYWORDS.save(deps.storage, keywords)?;
    }
    if let Some(favicon) = &msg.favicon {
        SITE_FAVICON.save(deps.storage, favicon)?;
    }

    Ok(Response::new().add_attribute("action", "instantiate"))
}
