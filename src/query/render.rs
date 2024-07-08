use serde_json::{Map as SerdeMap, Value};
use tinytemplate::TinyTemplate;

use crate::{
    error::ContractError,
    state::storage::{TEMPLATES, WEBSITE_TITLE},
};

use super::ReadonlyContext;

pub fn query_render(
    ctx: ReadonlyContext,
    path: String,
    context: Option<Value>,
) -> Result<String, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    // Initialize template renderer
    let mut template = TinyTemplate::new();
    let template_str = TEMPLATES.load(deps.storage, &path).map_err(|e| ContractError::Std(e))?;

    template
        .add_template(&path, &template_str)
        .map_err(|e| ContractError::TemplateError { reason: e.to_string() })?;

    // Initialize template rendering context object
    let mut map = SerdeMap::with_capacity(8);

    map.insert(
        "meta".to_owned(),
        Value::Object(SerdeMap::from_iter([
            ("title".to_owned(), Value::String(WEBSITE_TITLE.load(deps.storage)?)),
            ("path".to_owned(), Value::String(path.to_owned())),
        ])),
    );

    // Add context data from args
    if let Some(data) = context {
        map.insert("data".to_owned(), data);
    };

    // Render and return HTML
    template
        .render(&path, &Value::Object(map))
        .map_err(|e| ContractError::RenderError { reason: e.to_string() })
}
