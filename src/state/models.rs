use std::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint64};

#[cw_serde]
pub struct Config {
    pub rest_node: String,
}

#[cw_serde]
pub struct WebsiteMetadataContext {
    pub title: String,
    pub path: String,
}
#[cw_serde]
pub struct AssetInfo {
    pub name: String,
    pub mime_type: String,
}

#[cw_serde]
pub struct Asset {
    pub name: String,
    pub mime_type: String,
    pub data: Binary,
}

#[cw_serde]
pub struct Link {
    pub name: String,
    pub mime_type: String,
    pub uri: String,
}

#[cw_serde]
pub struct TemplateInfo {
    pub path: String,
    pub keywords: Vec<String>,
    pub scripts: Vec<String>,
    pub styles: Vec<String>,
}

#[cw_serde]
pub struct TemplateResponse {
    pub path: String,
    pub keywords: Vec<String>,
    pub scripts: Vec<String>,
    pub styles: Vec<String>,
    pub source: String,
}

#[cw_serde]
pub struct MetadataResponse {
    pub title: String,
    pub description: Option<String>,
    pub keywords: Vec<String>,
    pub favicon: Option<Link>,
}
