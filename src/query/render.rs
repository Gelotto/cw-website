use minijinja::{context, Environment};

use crate::{
    error::ContractError,
    msg::RenderParams,
    state::{
        models::WebsiteMetadata,
        storage::{TEMPLATES, WEBSITE_TITLE},
    },
};

use super::ReadonlyContext;

pub fn query_render(
    ctx: ReadonlyContext,
    path: String,
    params: Option<RenderParams>,
) -> Result<String, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    let mut jinja_env = Environment::new();

    let template_str = &TEMPLATES
        .load(deps.storage, &path)
        .map_err(|e| ContractError::Std(e))?;

    // Init template object
    jinja_env
        .add_template(&path, &template_str)
        .map_err(|e| ContractError::TemplateError {
            reason: e.to_string(),
        })?;

    let template = jinja_env
        .get_template(&path)
        .map_err(|e| ContractError::TemplateError {
            reason: e.to_string(),
        })?;

    let meta = WebsiteMetadata {
        title: WEBSITE_TITLE.load(deps.storage)?,
    };

    // Augment context with any route specific params
    let context = if let Some(params) = params {
        match params {
            RenderParams::Echo { message } => {
                context! {
                    meta => meta,
                    message => message,
                }
            },
        }
    } else {
        context! {
            meta => meta
        }
    };

    template
        .render(context)
        .map_err(|e| ContractError::RenderError {
            reason: e.to_string(),
        })
}
