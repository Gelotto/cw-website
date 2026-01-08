# CW-Website

A CosmWasm smart contract for rendering dynamic websites entirely on-chain via smart contract queries. Store HTML templates, CSS, and JavaScript on the blockchain and serve complete web pages through query endpoints.

## Overview

CW-Website enables fully decentralized website hosting by storing templates and assets directly in contract storage and rendering them dynamically with context variables. Unlike traditional IPFS-based hosting, content is queryable, mutable (by the admin), and can be integrated with other on-chain logic.

The contract uses TinyTemplate for server-side rendering and supports conditional asset injection, making it suitable for static sites, documentation, admin panels, and simple web applications.

## Features

- **HTML Templating**: TinyTemplate-based rendering with variable substitution
- **Static Asset Storage**: Store scripts and stylesheets on-chain with base64 encoding
- **Dynamic Rendering**: Inject context data at query time for personalized content
- **SEO Support**: Per-route and site-wide metadata (title, description, keywords, favicon)
- **Conditional Assets**: Optionally inject CSS/JS via client-side fetch for reduced query response size
- **Access Control**: Admin-only mutations prevent unauthorized content modification
- **HTML Escaping**: Automatic escaping of metadata to prevent injection attacks
- **Path Normalization**: Automatic conversion to absolute paths for consistent routing

## Architecture

### Storage Model

**Site Configuration:**
- `ADMIN` - Contract administrator (set during instantiation)
- `CONFIG` - REST API endpoint for asset queries
- `SITE_TITLE` - Global site title
- `SITE_DESCRIPTION` - Global meta description
- `SITE_KEYWORDS` - Site-wide SEO keywords
- `SITE_FAVICON` - Favicon link metadata

**Templates & Routes:**
- `ROUTE_TEMPLATES` - HTML template strings keyed by path
- `ROUTE_KEYWORDS` - Per-route SEO keywords
- `ROUTE_SCRIPT_NAMES` - Script assets to include for each route
- `ROUTE_STYLE_NAMES` - Style assets to include for each route

**Assets:**
- `SCRIPT_ASSETS` - JavaScript assets (base64-encoded)
- `STYLE_ASSETS` - CSS assets (base64-encoded)

### Entry Points

**Instantiate:**
```rust
pub struct InstantiateMsg {
    pub config: Config,          // REST node endpoint
    pub title: String,            // Site title (required)
    pub favicon: Option<Link>,    // Favicon metadata
    pub keywords: Option<Vec<String>>,
    pub description: Option<String>,
}
```

**Execute (Admin-Only):**
- `SetConfig(Config)` - Update REST node endpoint
- `SetMetadata { title, description, keywords, favicon }` - Update site metadata
- `Templates(Upsert)` - Add/update HTML template
- `Assets(Upsert)` - Add/update script or style asset

**Query (Public):**
- `Render { path, context, raw, inject }` - Render HTML page (primary query)
- `Metadata {}` - Get site metadata
- `Config {}` - Get configuration
- `Template { path }` - Get template info
- `Templates {}` - List all templates
- `Script { name }` / `Style { name }` - Get specific asset
- `Assets { path }` - Get all assets for a route

### Rendering Flow

1. **Path Resolution** - Normalize path to absolute form (`/path`)
2. **Template Loading** - Fetch template string from storage
3. **Context Building** - Create rendering context with `meta` (title, path) and optional `data`
4. **Rendering** - TinyTemplate renders variables into HTML
5. **Head Generation** - Build `<head>` with metadata, SEO tags, and favicon
6. **Asset Injection** - Optionally include client-side scripts to fetch CSS/JS
7. **Output** - Return complete HTML document or raw body

## Usage

### 1. Instantiate Contract

```bash
junod tx wasm instantiate <CODE_ID> '{
  "config": {
    "rest_node": "https://rest.cosmos.network"
  },
  "title": "My DApp",
  "description": "Decentralized application on Cosmos",
  "keywords": ["cosmos", "dapp", "blockchain"],
  "favicon": {
    "name": "favicon",
    "mime_type": "image/png",
    "uri": "/favicon.png"
  }
}' --from admin --label "my-website"
```

### 2. Upload Template

```bash
junod tx wasm execute <CONTRACT_ADDR> '{
  "templates": {
    "upsert": {
      "path": "/",
      "template": "<body><h1>{meta.title}</h1><p>Welcome to {data.project}!</p></body>",
      "keywords": ["homepage", "welcome"],
      "scripts": ["main"],
      "styles": ["theme"]
    }
  }
}' --from admin
```

### 3. Upload Assets

**CSS:**
```bash
# Base64 encode your CSS first
CSS_DATA=$(cat theme.css | base64)

junod tx wasm execute <CONTRACT_ADDR> '{
  "assets": {
    "upsert": {
      "name": "theme",
      "asset_type": "style",
      "mime_type": "text/css",
      "data": "'$CSS_DATA'"
    }
  }
}' --from admin
```

**JavaScript:**
```bash
# Base64 encode your JS first
JS_DATA=$(cat main.js | base64)

junod tx wasm execute <CONTRACT_ADDR> '{
  "assets": {
    "upsert": {
      "name": "main",
      "asset_type": "script",
      "mime_type": "text/javascript",
      "data": "'$JS_DATA'"
    }
  }
}' --from admin
```

### 4. Render Page

**Query:**
```bash
junod query wasm contract-state smart <CONTRACT_ADDR> '{
  "render": {
    "path": "/",
    "context": {
      "project": "Cosmos"
    }
  }
}'
```

**Response:**
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="description" content="Decentralized application on Cosmos">
  <meta name="keywords" content="cosmos,dapp,blockchain,homepage,welcome">
  <title>My DApp</title>
  <link rel="icon" type="image/png" href="/favicon.png">
</head>
<body><h1>My DApp</h1><p>Welcome to Cosmos!</p></body>
</html>
```

## Template Variables

Templates have access to the following context:

- `meta.title` - Site title from contract storage
- `meta.path` - Current route path being rendered
- `data.*` - Any custom data passed via the `context` parameter in the query

**Example:**
```html
<body>
  <h1>{meta.title}</h1>
  <p>Current page: {meta.path}</p>
  <p>User: {data.username}</p>
</body>
```

## Query Parameters

The `Render` query supports optional parameters:

- `raw` (default: `false`) - If `true`, returns only the rendered template body without HTML wrapper
- `inject` (default: `false`) - If `true`, includes client-side scripts to fetch and inject CSS/JS assets dynamically
- `context` (default: `null`) - JSON object with custom data for template variable substitution

**Example with all parameters:**
```json
{
  "render": {
    "path": "/dashboard",
    "context": {
      "user": "alice",
      "balance": "1000"
    },
    "raw": false,
    "inject": true
  }
}
```

## Security

### Access Control

The contract implements owner-based access control:
- The **instantiator** becomes the **admin**
- Only the admin can execute state-changing operations:
  - `SetConfig`
  - `SetMetadata`
  - `Templates::Upsert`
  - `Assets::Upsert`
- All queries are public and permissionless

### HTML Escaping

Site metadata (title, description) is automatically HTML-escaped when rendered in the `<head>` section to prevent injection attacks. Template bodies are **not** escaped, allowing admins full control over HTML structure.

**Escaped in `<head>`:**
- Site title
- Meta description

**Not escaped (admin controls content):**
- Template body
- Keywords (inserted as comma-separated list)

## Integration with On-Chain Stack

CW-Website is designed to work alongside complementary contracts:

### cw-factory
Acts as the backend database layer. Factory can manage multiple contract instances with multi-indexing, allowing CW-Website to query and display catalogs of on-chain data (e.g., list of DAOs, tokens, or user accounts).

**Use Case:** Website queries factory for list of managed contracts, renders them as a directory page.

### cw-acl
Provides hierarchical access control. While CW-Website uses simple admin-only permissions, integrating with CW-ACL enables more granular control (e.g., different admins for different routes, time-bound publishing permissions).

**Use Case:** Query ACL to determine which UI elements to display based on user permissions.

### Full Stack Vision

```
┌─────────────────────────────────────────┐
│  Frontend (cw-website)                  │
│  HTML templates + CSS/JS assets         │
└─────────────────────────────────────────┘
           ↓ queries
┌─────────────────────────────────────────┐
│  Backend (cw-factory)                   │
│  Multi-indexed contract instances       │
└─────────────────────────────────────────┘
           ↓ permission checks
┌─────────────────────────────────────────┐
│  Authorization (cw-acl)                 │
│  Hierarchical access control            │
└─────────────────────────────────────────┘
```

This enables **fully on-chain web applications** with no off-chain dependencies for hosting, databases, or authentication.

## Development

### Build

```bash
./bin/build
```

This creates an optimized WASM binary using Docker and `rust-optimizer`.

### Test

```bash
cargo test
```

The test suite includes 27+ tests covering:
- Authorization (admin-only access)
- Instantiation (full and minimal)
- Queries (config, metadata, templates)
- Template operations (upsert, keywords, path normalization)
- Rendering (basic, context variables, raw mode, HTML escaping, keyword merging)
- Asset management (scripts, styles, route assets)

### Deploy

**Deploy to devnet:**
```bash
./bin/deploy devnet <sender-address>
```

**Deploy to testnet:**
```bash
./bin/deploy testnet <sender-address>
```

**Deploy to mainnet:**
```bash
./bin/deploy mainnet <sender-address>
```

**Instantiate:**
```bash
./bin/instantiate <network> <sender-address>
```

## Schema

Generate JSON schema for messages:

```bash
cargo schema
```

Schemas are output to `schema/` directory.

## Examples

### Simple Static Website

```json
// Instantiate
{
  "config": { "rest_node": "https://rest.cosmos.network" },
  "title": "Cosmos Documentation",
  "description": "Learn about the Cosmos ecosystem"
}

// Add homepage
{
  "templates": {
    "upsert": {
      "path": "/",
      "template": "<body><h1>Welcome</h1><nav><a href='/docs'>Docs</a></nav></body>"
    }
  }
}

// Add docs page
{
  "templates": {
    "upsert": {
      "path": "/docs",
      "template": "<body><h1>Documentation</h1><p>Getting started...</p></body>"
    }
  }
}
```

### Dynamic Dashboard

```json
// Template with variables
{
  "templates": {
    "upsert": {
      "path": "/dashboard",
      "template": "<body><h1>Dashboard</h1><p>Balance: {data.balance}</p><p>Staked: {data.staked}</p></body>"
    }
  }
}

// Render with user data
{
  "render": {
    "path": "/dashboard",
    "context": {
      "balance": "1000 ATOM",
      "staked": "500 ATOM"
    }
  }
}
```

## Limitations

- **Asset Size**: Assets are stored on-chain, making large files (>100KB) expensive. Suitable for minimized CSS/JS only.
- **Template Complexity**: TinyTemplate is intentionally simple. Complex logic should be handled off-chain or in separate contracts.
- **No Server-Side Routing**: Each route must be explicitly defined. Wildcard routing not supported.
- **Single Admin**: Only one admin per contract instance. For multi-admin scenarios, consider deploying multiple instances or integrating cw-acl.

## License

Apache-2.0

## Contributing

This contract is part of the on-chain fullstack stack. For issues or contributions, please coordinate with the broader ecosystem development.
