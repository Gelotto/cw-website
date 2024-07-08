use cosmwasm_schema::cw_serde;
use serde_json::Value;

use crate::state::models::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub title: String,
}

#[cw_serde]
pub enum TemplatesExecuteMsg {
    Upsert { path: String, template: String },
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
    Templates(TemplatesExecuteMsg),
}

#[cw_serde]
pub struct ContextValue {
    pub name: String,
    pub value: Value,
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Render { path: String, context: Option<Value> },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);
