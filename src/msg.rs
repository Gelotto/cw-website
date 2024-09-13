use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use serde_json::Value;

use crate::state::models::{AssetInfo, AssetType, Config, Link, TemplateInfo};

#[cw_serde]
pub struct InstantiateMsg {
    pub config: Config,
    pub title: String,
    pub favicon: Option<Link>,
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
}

#[cw_serde]
pub enum AssetsExecuteMsg {
    Upsert {
        name: String,
        asset_type: AssetType,
        mime_type: String,
        data: Binary,
    },
}

#[cw_serde]
pub enum TemplatesExecuteMsg {
    Upsert {
        path: String,
        template: String,
        scripts: Option<Vec<String>>,
        styles: Option<Vec<String>>,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
    SetMetadata {
        title: String,
        description: Option<String>,
        keywords: Option<Vec<String>>,
        favicon: Option<Link>,
    },
    Templates(TemplatesExecuteMsg),
    Assets(AssetsExecuteMsg),
}

#[cw_serde]
pub struct ContextValue {
    pub name: String,
    pub value: Value,
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    Metadata {},
    Render {
        path: String,
        context: Option<Value>,
        raw: Option<bool>,
        inject: Option<bool>,
    },
    Script {
        name: String,
    },
    Style {
        name: String,
    },
    Assets {
        path: String,
    },
    Template {
        path: String,
    },
    Templates {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct TemplatesResponse(pub Vec<TemplateInfo>);

#[cw_serde]
pub struct TemplateAssetsResponse {
    pub scripts: Vec<AssetInfo>,
    pub styles: Vec<AssetInfo>,
}
