use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_json, Binary};

use crate::contract::{execute, instantiate, query};
use crate::error::ContractError;
use crate::msg::{
    AssetsExecuteMsg, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TemplatesExecuteMsg,
};
use crate::state::models::{AssetType, Config, Link, MetadataResponse};

// Test constants
const ADMIN: &str = "admin";
const USER: &str = "user";

// Helper function to create a valid instantiate message
fn create_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg {
        config: Config {
            rest_node: "http://localhost:1317".to_string(),
        },
        title: "Test Website".to_string(),
        description: Some("A test website on the blockchain".to_string()),
        keywords: Some(vec!["test".to_string(), "blockchain".to_string()]),
        favicon: Some(Link {
            name: "favicon".to_string(),
            mime_type: "image/png".to_string(),
            uri: "/favicon.png".to_string(),
        }),
    }
}

// Helper function to create minimal instantiate message
fn create_minimal_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg {
        config: Config {
            rest_node: "http://localhost:1317".to_string(),
        },
        title: "Test Website".to_string(),
        description: None,
        keywords: None,
        favicon: None,
    }
}

// ============================================================================
// Authorization Tests
// ============================================================================

#[test]
fn test_instantiate_sets_admin() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    let res = instantiate(deps.as_mut(), env.clone(), info, msg);
    assert!(res.is_ok());

    // Verify admin can execute SetConfig
    let set_config_msg = ExecuteMsg::SetConfig(Config {
        rest_node: "http://new-node:1317".to_string(),
    });
    let info = mock_info(ADMIN, &[]);
    let res = execute(deps.as_mut(), env, info, set_config_msg);
    assert!(res.is_ok());
}

#[test]
fn test_only_admin_can_set_config() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Non-admin tries to set config
    let set_config_msg = ExecuteMsg::SetConfig(Config {
        rest_node: "http://malicious:1317".to_string(),
    });
    let info = mock_info(USER, &[]);
    let res = execute(deps.as_mut(), env, info, set_config_msg);

    assert!(res.is_err());
    match res.unwrap_err() {
        ContractError::NotAuthorized { .. } => {}
        e => panic!("Expected NotAuthorized error, got: {:?}", e),
    }
}

#[test]
fn test_only_admin_can_set_metadata() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Non-admin tries to set metadata
    let set_metadata_msg = ExecuteMsg::SetMetadata {
        title: "Hacked".to_string(),
        description: None,
        keywords: None,
        favicon: None,
    };
    let info = mock_info(USER, &[]);
    let res = execute(deps.as_mut(), env, info, set_metadata_msg);

    assert!(res.is_err());
    match res.unwrap_err() {
        ContractError::NotAuthorized { .. } => {}
        e => panic!("Expected NotAuthorized error, got: {:?}", e),
    }
}

#[test]
fn test_only_admin_can_upsert_template() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Non-admin tries to upsert template
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/".to_string(),
        template: "<body>Hacked</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    let info = mock_info(USER, &[]);
    let res = execute(deps.as_mut(), env, info, upsert_msg);

    assert!(res.is_err());
    match res.unwrap_err() {
        ContractError::NotAuthorized { .. } => {}
        e => panic!("Expected NotAuthorized error, got: {:?}", e),
    }
}

#[test]
fn test_only_admin_can_upsert_asset() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    // Non-admin tries to upsert asset
    let upsert_msg = ExecuteMsg::Assets(AssetsExecuteMsg::Upsert {
        name: "malicious".to_string(),
        asset_type: AssetType::Script,
        mime_type: "text/javascript".to_string(),
        data: Binary::from(b"alert('hacked')"),
    });
    let info = mock_info(USER, &[]);
    let res = execute(deps.as_mut(), env, info, upsert_msg);

    assert!(res.is_err());
    match res.unwrap_err() {
        ContractError::NotAuthorized { .. } => {}
        e => panic!("Expected NotAuthorized error, got: {:?}", e),
    }
}

#[test]
fn test_admin_can_execute_all_operations() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Admin can set config
    let set_config_msg = ExecuteMsg::SetConfig(Config {
        rest_node: "http://new-node:1317".to_string(),
    });
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), set_config_msg).is_ok());

    // Admin can set metadata
    let set_metadata_msg = ExecuteMsg::SetMetadata {
        title: "Updated Title".to_string(),
        description: Some("Updated description".to_string()),
        keywords: Some(vec!["updated".to_string()]),
        favicon: None,
    };
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), set_metadata_msg).is_ok());

    // Admin can upsert template
    let upsert_template_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/".to_string(),
        template: "<body>Hello</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    assert!(execute(deps.as_mut(), env.clone(), info.clone(), upsert_template_msg).is_ok());

    // Admin can upsert asset
    let upsert_asset_msg = ExecuteMsg::Assets(AssetsExecuteMsg::Upsert {
        name: "main".to_string(),
        asset_type: AssetType::Script,
        mime_type: "text/javascript".to_string(),
        data: Binary::from(b"console.log('test')"),
    });
    assert!(execute(deps.as_mut(), env, info, upsert_asset_msg).is_ok());
}

// ============================================================================
// Instantiation Tests
// ============================================================================

#[test]
fn test_instantiate_with_full_metadata() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    let res = instantiate(deps.as_mut(), env.clone(), info, msg);
    assert!(res.is_ok());

    // Query metadata to verify all fields were saved
    let query_msg = QueryMsg::Metadata {};
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let metadata: MetadataResponse = from_json(&res).unwrap();

    assert_eq!(metadata.title, "Test Website");
    assert_eq!(
        metadata.description,
        Some("A test website on the blockchain".to_string())
    );
    assert_eq!(
        metadata.keywords,
        vec!["test".to_string(), "blockchain".to_string()]
    );
    assert!(metadata.favicon.is_some());
}

#[test]
fn test_instantiate_minimal() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_minimal_instantiate_msg();

    let res = instantiate(deps.as_mut(), env.clone(), info, msg);
    assert!(res.is_ok());

    // Query metadata to verify required fields were saved
    let query_msg = QueryMsg::Metadata {};
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let metadata: MetadataResponse = from_json(&res).unwrap();

    assert_eq!(metadata.title, "Test Website");
    assert_eq!(metadata.description, None);
    assert_eq!(metadata.keywords, Vec::<String>::new());
    assert_eq!(metadata.favicon, None);
}

// ============================================================================
// Query Tests
// ============================================================================

#[test]
fn test_query_config_returns_correct_type() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let query_msg = QueryMsg::Config {};
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let config: ConfigResponse = from_json(&res).unwrap();

    assert_eq!(config.0.rest_node, "http://localhost:1317");
}

#[test]
fn test_query_metadata_returns_correct_type() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

    let query_msg = QueryMsg::Metadata {};
    let res = query(deps.as_ref(), env, query_msg).unwrap();

    // This should deserialize successfully to MetadataResponse, not ConfigResponse
    let metadata: MetadataResponse = from_json(&res).unwrap();
    assert_eq!(metadata.title, "Test Website");
}

#[test]
fn test_query_templates_list() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add a template
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/".to_string(),
        template: "<body>Home</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Query templates
    let query_msg = QueryMsg::Templates {};
    let res = query(deps.as_ref(), env, query_msg);
    assert!(res.is_ok());
}

// ============================================================================
// Template Tests
// ============================================================================

#[test]
fn test_upsert_basic_template() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/about".to_string(),
        template: "<body>About us</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });

    let res = execute(deps.as_mut(), env, info, upsert_msg);
    assert!(res.is_ok());
}

#[test]
fn test_upsert_template_with_keywords() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/products".to_string(),
        template: "<body>Products</body>".to_string(),
        keywords: Some(vec!["products".to_string(), "shop".to_string()]),
        scripts: None,
        styles: None,
    });

    let res = execute(deps.as_mut(), env, info, upsert_msg);
    assert!(res.is_ok());
}

#[test]
fn test_upsert_template_with_scripts_and_styles() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/interactive".to_string(),
        template: "<body>Interactive page</body>".to_string(),
        keywords: None,
        scripts: Some(vec!["main.js".to_string()]),
        styles: Some(vec!["theme.css".to_string()]),
    });

    let res = execute(deps.as_mut(), env, info, upsert_msg);
    assert!(res.is_ok());
}

#[test]
fn test_query_single_template() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add template
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/contact".to_string(),
        template: "<body>Contact us</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Query the template
    let query_msg = QueryMsg::Template {
        path: "/contact".to_string(),
    };
    let res = query(deps.as_ref(), env, query_msg);
    assert!(res.is_ok());
}

#[test]
fn test_path_normalization() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Insert template without leading slash
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "test".to_string(), // No leading slash
        template: "<body>Test</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Query with leading slash - should work due to normalization
    let query_msg = QueryMsg::Template {
        path: "/test".to_string(),
    };
    let res = query(deps.as_ref(), env, query_msg);
    assert!(res.is_ok());
}

// ============================================================================
// Rendering Tests
// ============================================================================

#[test]
fn test_render_basic_template() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add template
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/".to_string(),
        template: "<body>Welcome!</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Render the page
    let query_msg = QueryMsg::Render {
        path: "/".to_string(),
        context: None,
        raw: Some(false),
        inject: Some(false),
    };
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let html: String = from_json(&res).unwrap();

    assert!(html.contains("<!DOCTYPE html>"));
    assert!(html.contains("<title>Test Website</title>"));
    assert!(html.contains("<body>Welcome!</body>"));
}

#[test]
fn test_render_with_context() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add template with variable
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/greet".to_string(),
        template: "<body>Hello {data.name}!</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Render with context
    let context = serde_json::json!({
        "name": "World"
    });
    let query_msg = QueryMsg::Render {
        path: "/greet".to_string(),
        context: Some(context),
        raw: Some(false),
        inject: Some(false),
    };
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let html: String = from_json(&res).unwrap();

    assert!(html.contains("Hello World!"));
}

#[test]
fn test_render_raw_mode() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add template
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/raw".to_string(),
        template: "<body>Raw content</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Render in raw mode
    let query_msg = QueryMsg::Render {
        path: "/raw".to_string(),
        context: None,
        raw: Some(true),
        inject: Some(false),
    };
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let html: String = from_json(&res).unwrap();

    // Raw mode should NOT include DOCTYPE or head
    assert!(!html.contains("<!DOCTYPE html>"));
    assert!(!html.contains("<head>"));
    assert_eq!(html, "<body>Raw content</body>");
}

#[test]
fn test_render_html_escaping() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);

    // Create instantiate message with potentially dangerous characters
    let msg = InstantiateMsg {
        config: Config {
            rest_node: "http://localhost:1317".to_string(),
        },
        title: "Test <script>alert('xss')</script>".to_string(),
        description: Some("Description with \"quotes\" and <tags>".to_string()),
        keywords: None,
        favicon: None,
    };

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add template
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/".to_string(),
        template: "<body>Test</body>".to_string(),
        keywords: None,
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Render the page
    let query_msg = QueryMsg::Render {
        path: "/".to_string(),
        context: None,
        raw: Some(false),
        inject: Some(false),
    };
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let html: String = from_json(&res).unwrap();

    // Verify HTML entities are escaped in title
    assert!(html.contains("&lt;script&gt;"));
    assert!(html.contains("&lt;/script&gt;"));

    // Verify HTML entities are escaped in description
    assert!(html.contains("&quot;quotes&quot;"));
    assert!(html.contains("&lt;tags&gt;"));
}

#[test]
fn test_render_keywords_merge() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg(); // Has site-wide keywords: ["test", "blockchain"]

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add template with route-specific keywords
    let upsert_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/special".to_string(),
        template: "<body>Special page</body>".to_string(),
        keywords: Some(vec!["special".to_string(), "unique".to_string()]),
        scripts: None,
        styles: None,
    });
    execute(deps.as_mut(), env.clone(), info, upsert_msg).unwrap();

    // Render the page
    let query_msg = QueryMsg::Render {
        path: "/special".to_string(),
        context: None,
        raw: Some(false),
        inject: Some(false),
    };
    let res = query(deps.as_ref(), env, query_msg).unwrap();
    let html: String = from_json(&res).unwrap();

    // Should contain both site-wide and route-specific keywords
    assert!(html.contains(r#"<meta name="keywords" content="test,blockchain,special,unique">"#));
}

// ============================================================================
// Asset Tests
// ============================================================================

#[test]
fn test_upsert_script_asset() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let script_content = b"console.log('Hello from blockchain');";
    let upsert_msg = ExecuteMsg::Assets(AssetsExecuteMsg::Upsert {
        name: "main.js".to_string(),
        asset_type: AssetType::Script,
        mime_type: "text/javascript".to_string(),
        data: Binary::from(script_content),
    });

    let res = execute(deps.as_mut(), env.clone(), info, upsert_msg);
    assert!(res.is_ok());

    // Query the script
    let query_msg = QueryMsg::Script {
        name: "main.js".to_string(),
    };
    let res = query(deps.as_ref(), env, query_msg);
    assert!(res.is_ok());
}

#[test]
fn test_upsert_style_asset() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    let css_content = b"body { background: blue; }";
    let upsert_msg = ExecuteMsg::Assets(AssetsExecuteMsg::Upsert {
        name: "theme.css".to_string(),
        asset_type: AssetType::Style,
        mime_type: "text/css".to_string(),
        data: Binary::from(css_content),
    });

    let res = execute(deps.as_mut(), env.clone(), info, upsert_msg);
    assert!(res.is_ok());

    // Query the style
    let query_msg = QueryMsg::Style {
        name: "theme.css".to_string(),
    };
    let res = query(deps.as_ref(), env, query_msg);
    assert!(res.is_ok());
}

#[test]
fn test_query_assets_for_route() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let info = mock_info(ADMIN, &[]);
    let msg = create_instantiate_msg();

    instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

    // Add assets
    let script_msg = ExecuteMsg::Assets(AssetsExecuteMsg::Upsert {
        name: "app.js".to_string(),
        asset_type: AssetType::Script,
        mime_type: "text/javascript".to_string(),
        data: Binary::from(b"// app code"),
    });
    execute(deps.as_mut(), env.clone(), info.clone(), script_msg).unwrap();

    let style_msg = ExecuteMsg::Assets(AssetsExecuteMsg::Upsert {
        name: "app.css".to_string(),
        asset_type: AssetType::Style,
        mime_type: "text/css".to_string(),
        data: Binary::from(b"/* styles */"),
    });
    execute(deps.as_mut(), env.clone(), info.clone(), style_msg).unwrap();

    // Add template that references these assets
    let template_msg = ExecuteMsg::Templates(TemplatesExecuteMsg::Upsert {
        path: "/app".to_string(),
        template: "<body>App</body>".to_string(),
        keywords: None,
        scripts: Some(vec!["app.js".to_string()]),
        styles: Some(vec!["app.css".to_string()]),
    });
    execute(deps.as_mut(), env.clone(), info, template_msg).unwrap();

    // Query assets for this route
    let query_msg = QueryMsg::Assets {
        path: "/app".to_string(),
    };
    let res = query(deps.as_ref(), env, query_msg);
    assert!(res.is_ok());
}
