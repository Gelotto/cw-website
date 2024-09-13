use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use cosmwasm_std::Addr;
use serde_json::{Map as SerdeMap, Value};
use tinytemplate::TinyTemplate;

use crate::{
    error::ContractError,
    state::{
        models::Link,
        storage::{
            CONFIG, ROUTE_KEYWORDS, ROUTE_SCRIPT_NAMES, ROUTE_STYLE_NAMES, ROUTE_TEMPLATES, SITE_DESCRIPTION,
            SITE_FAVICON, SITE_KEYWORDS, SITE_TITLE,
        },
    },
};

use super::ReadonlyContext;

const FAVICON_PATH: &str = "favicon.ico";

pub fn query_render(
    ctx: ReadonlyContext,
    path: String,
    context: Option<Value>,
    raw: Option<bool>,
    inject_assets: Option<bool>,
) -> Result<String, ContractError> {
    let ReadonlyContext { deps, env, .. } = ctx;
    let inject_assets = inject_assets.unwrap_or_default();

    // Ensure all path is absolute
    let mut path = path;
    if !path.starts_with("/") {
        path = format!("/{}", path);
    }

    if path == FAVICON_PATH {
        return Ok(SITE_FAVICON
            .may_load(deps.storage)?
            .and_then(|link| Some(link.uri.to_owned()))
            .unwrap_or_default());
    }

    // Initialize template renderer
    let mut template = TinyTemplate::new();
    let template_str = ROUTE_TEMPLATES
        .load(deps.storage, &path)
        .map_err(|e| ContractError::Std(e))?;

    template
        .add_template(&path, &template_str)
        .map_err(|e| ContractError::TemplateError { reason: e.to_string() })?;

    // Get static text for <title> tag
    let title = SITE_TITLE.load(deps.storage)?;

    // Merge route-specific keywords into site-global keywords
    let mut keywords: Vec<String> = SITE_KEYWORDS.load(deps.storage).unwrap_or_default();
    keywords.extend(ROUTE_KEYWORDS.may_load(deps.storage, &path)?.unwrap_or_default());

    // Initialize template rendering context object
    let mut map = SerdeMap::with_capacity(8);
    map.insert(
        "meta".to_owned(),
        Value::Object(SerdeMap::from_iter([
            ("title".to_owned(), Value::String(title.to_owned())),
            ("path".to_owned(), Value::String(path.to_owned())),
        ])),
    );

    // Add context data from args
    if let Some(data) = context {
        map.insert("data".to_owned(), data);
    };

    let config = CONFIG.load(deps.storage)?;

    // Render HTML <body>
    let body = template
        .render(&path, &Value::Object(map))
        .map_err(|e| ContractError::RenderError { reason: e.to_string() })?;

    // Render HTML <head>
    if raw.unwrap_or(false) {
        Ok(body)
    } else {
        let head = render_head(
            &config.rest_node,
            &env.contract.address,
            title,
            keywords,
            SITE_DESCRIPTION.may_load(deps.storage)?,
            SITE_FAVICON.may_load(deps.storage)?,
            ROUTE_STYLE_NAMES.may_load(deps.storage, &path).unwrap_or_default(),
            ROUTE_SCRIPT_NAMES.may_load(deps.storage, &path).unwrap_or_default(),
            inject_assets,
        );

        // Return full HTML doc
        Ok(format!(
            "<!DOCTYPE html>\n<html lang=\"en\">\n{}\n{}\n</html>",
            head, body
        ))
    }
}

fn render_head(
    host: &String,
    contract: &Addr,
    title: String,
    keywords: Vec<String>,
    maybe_description: Option<String>,
    maybe_favicon: Option<Link>,
    maybe_stylesheet_names: Option<Vec<String>>,
    maybe_script_names: Option<Vec<String>>,
    inject_assets: bool,
) -> String {
    let mut template = String::from("<head>\n");

    template.push_str(r#"<meta charset="UTF-8">"#);
    template.push_str("\n");

    template.push_str(r#"<meta name="robots">"#);
    template.push_str("\n");

    template.push_str(r#"<meta http-equiv="Content-type" content="text/html; charset=UTF-8">"#);
    template.push_str("\n");

    template.push_str(r#"<meta http-equiv="X-UA-Compatible" content="IE=edge">"#);
    template.push_str("\n");

    template.push_str(r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#);
    template.push_str("\n");

    if let Some(description) = maybe_description {
        template.push_str(&format!(r#"<meta name="description" content="{}">"#, description));
        template.push_str("\n");
    }

    template.push_str(&format!(r#"<title>{}</title>"#, title));
    template.push_str("\n");

    if let Some(favicon) = maybe_favicon {
        template.push_str(&format!(
            r#"<link rel="icon" type="{}", href={}>"#,
            favicon.mime_type, favicon.uri
        ));
        template.push_str("\n");
    }
    if !keywords.is_empty() {
        let content = keywords.join(",");
        template.push_str(&format!(r#"<meta name="keywords" content="{}">"#, content));
        template.push_str("\n");
    }

    if inject_assets {
        if let Some(names) = maybe_stylesheet_names {
            for (i, name) in names.iter().enumerate() {
                let id = format!("_styleInjector{}", i);
                let injection_script = inject_css(id, host, contract, &name);
                template.push_str(&injection_script);
                template.push_str("\n");
            }
        }
        if let Some(names) = maybe_script_names {
            for (i, name) in names.iter().enumerate() {
                let id = format!("_scriptInjector{}", i);
                let injection_script = inject_script(id, host, contract, &name);
                template.push_str(&injection_script);
                template.push_str("\n");
            }
        }
    }
    template.push_str(r#"</head>"#);
    template.push_str("\n");
    template
}

fn inject_css(
    id: String,
    host: &String,
    contract: &Addr,
    name: &String,
) -> String {
    let query_msg = format!(r#"{{"asset": {{"name": "{}"}}}}"#, name);
    let url_path = format!(
        "/cosmwasm/wasm/v1/contract/{}/smart/{}",
        contract.to_string(),
        URL_SAFE.encode(query_msg.as_bytes())
    );
    format!(
        r#"<script type="text/javascript" id="{}" async>
            const host = "{}";
            const url = `${{host}}{}`;
            try {{
                fetch(url).then((resp) => {{
                    resp.json().then((json) => {{
                        const css = atob(json.data.data);
                        const el = document.createElement('style');
                        el.setAttribute("type", "text/css");
                        el.innerHTML = css
                        document.head.appendChild(el);
                    }})
                }})
                document.head.removeChild(
                    document.getElementById("{}")
                );
            }} catch (e) {{
             console.error(e);
        }}
        </script>"#,
        id, host, url_path, id
    )
}

fn inject_script(
    id: String,
    host: &String,
    contract: &Addr,
    name: &String,
) -> String {
    let query_msg = format!(r#"{{"asset": {{"name": "{}"}}}}"#, name);
    let url_path = format!(
        "/cosmwasm/wasm/v1/contract/{}/smart/{}",
        contract.to_string(),
        URL_SAFE.encode(query_msg.as_bytes())
    );
    format!(
        r#"<script type="text/javascript" id="{}" async>
            const host = "{}";
            const url = `${{host}}{}`;
            try {{
                fetch(url).then((resp) => {{
                    resp.json().then((json) => {{
                        const el = document.createElement('script');
                        const code = atob(json.data.data);
                        el.innerHTML = code
                        document.body.appendChild(el);
                    }})
                }})
                document.head.removeChild(
                    document.getElementById("{}")
                );
            }} catch (e) {{
                console.error(e);
            }}
        </script>"#,
        id, host, url_path, id
    )
}
