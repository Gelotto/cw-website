use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct Config {}

#[cw_serde]
pub struct WebsiteMetadata {
    pub title: String,
}
