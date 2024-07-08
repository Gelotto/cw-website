use std::fmt;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct WebsiteMetadataContext {
    pub title: String,
    pub path: String,
}
