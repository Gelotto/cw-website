pub mod models;
pub mod storage;

use cosmwasm_std::Response;
use storage::WEBSITE_TITLE;

use crate::{error::ContractError, execute::Context, msg::InstantiateMsg};

/// Top-level initialization of contract state
pub fn init(
    ctx: Context,
    msg: &InstantiateMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;

    WEBSITE_TITLE.save(deps.storage, &msg.title)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}
