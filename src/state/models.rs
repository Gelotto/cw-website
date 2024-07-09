use std::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Uint64};

#[cw_serde]
pub struct Config {
    pub rest: String,
}

#[cw_serde]
pub struct WebsiteMetadataContext {
    pub title: String,
    pub path: String,
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
