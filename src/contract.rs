use crate::error::ContractError;
use crate::execute::set_metadata::exec_set_metadata;
use crate::execute::upsert_asset::exec_upsert_asset;
use crate::execute::upsert_template::exec_upsert_template;
use crate::execute::{set_config::exec_set_config, Context};
use crate::msg::{AssetsExecuteMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, TemplatesExecuteMsg};
use crate::query::asset::query_asset;
use crate::query::assets::query_assets;
use crate::query::render::query_render;
use crate::query::template::query_template;
use crate::query::templates::query_templates;
use crate::query::{config::query_config, ReadonlyContext};
use crate::state;
use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(state::init(Context { deps, env, info }, &msg)?)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let ctx = Context { deps, env, info };
    match msg {
        // Set config params, like the default REST API host, etc.
        ExecuteMsg::SetConfig(config) => exec_set_config(ctx, config),

        // Set the site metadata rendered in the <head> of each page's HTML
        ExecuteMsg::SetMetadata {
            description,
            favicon,
            keywords,
            title,
        } => exec_set_metadata(ctx, title, description, keywords, favicon),

        // Template-related executions
        ExecuteMsg::Templates(msg) => match msg {
            TemplatesExecuteMsg::Upsert {
                path,
                template,
                scripts,
                styles,
            } => exec_upsert_template(ctx, path, template, scripts, styles),
        },

        // Asset-related executions
        ExecuteMsg::Assets(msg) => match msg {
            AssetsExecuteMsg::Upsert { name, mime_type, data } => exec_upsert_asset(ctx, name, mime_type, data),
        },
    }
}

#[entry_point]
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> Result<Binary, ContractError> {
    let ctx = ReadonlyContext { deps, env };
    let result = match msg {
        QueryMsg::Render { path, context } => to_json_binary(&query_render(ctx, path, context)?),
        QueryMsg::Config {} => to_json_binary(&query_config(ctx)?),
        QueryMsg::Metadata {} => to_json_binary(&query_config(ctx)?),
        QueryMsg::Asset { name } => to_json_binary(&query_asset(ctx, name)?),
        QueryMsg::Assets {} => to_json_binary(&query_assets(ctx)?),
        QueryMsg::Template { path } => to_json_binary(&query_template(ctx, path)?),
        QueryMsg::Templates {} => to_json_binary(&query_templates(ctx)?),
    }?;
    Ok(result)
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
