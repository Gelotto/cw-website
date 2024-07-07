use cosmwasm_schema::cw_serde;

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
pub enum RenderParams {
    Echo { message: String },
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Render {
        path: String,
        params: Option<RenderParams>,
    },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);
