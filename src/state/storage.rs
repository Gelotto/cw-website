use cw_storage_plus::{Item, Map};

use super::models::Config;

pub const CONFIG: Item<Config> = Item::new("config");
pub const TEMPLATES: Map<&String, String> = Map::new("templates");
pub const WEBSITE_TITLE: Item<String> = Item::new("website_title");
