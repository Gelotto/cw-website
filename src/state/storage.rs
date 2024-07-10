use cw_storage_plus::{Item, Map};

use super::models::{Asset, Config, Link};

pub const CONFIG: Item<Config> = Item::new("config");

pub const SITE_TITLE: Item<String> = Item::new("website_title");
pub const SITE_FAVICON: Item<Link> = Item::new("website_favicon");
pub const SITE_KEYWORDS: Item<Vec<String>> = Item::new("site_keywords");
pub const SITE_DESCRIPTION: Item<String> = Item::new("site_description");

pub const ASSETS: Map<&String, Asset> = Map::new("assets");

pub const ROUTE_TEMPLATES: Map<&String, String> = Map::new("route_templates");
pub const ROUTE_KEYWORDS: Map<&String, Vec<String>> = Map::new("route_keywords");
pub const ROUTE_STYLE_NAMES: Map<&String, Vec<String>> = Map::new("route_style_names");
pub const ROUTE_SCRIPT_NAMES: Map<&String, Vec<String>> = Map::new("route_script_names");
