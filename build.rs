//! Build script for code generation
//!
//! This script generates Rust code from OpenAPI schemas during build time.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=forum.json");
    println!("cargo:rerun-if-changed=market.json");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let forum_path = PathBuf::from(&manifest_dir).join("forum.json");
    let market_path = PathBuf::from(&manifest_dir).join("market.json");
    let generated_dir = PathBuf::from(&manifest_dir).join("src").join("generated");

    // Ensure generated directory exists
    fs::create_dir_all(&generated_dir).expect("Failed to create generated directory");

    // Generate code for Forum API
    if forum_path.exists() {
        println!("cargo:warning=Generating Forum API code...");
        generate_api_code(&forum_path, &generated_dir, "forum");
    }

    // Generate code for Market API
    if market_path.exists() {
        println!("cargo:warning=Generating Market API code...");
        generate_api_code(&market_path, &generated_dir, "market");
    }

    println!("cargo:warning=Code generation completed!");
}

fn generate_api_code(schema_path: &Path, output_dir: &Path, api_name: &str) {
    // Read the OpenAPI schema
    let content = fs::read_to_string(schema_path).unwrap_or_else(|e| {
        println!("cargo:warning=Failed to read schema: {}", e);
        String::new()
    });

    if content.is_empty() {
        return;
    }

    // Parse JSON
    let schema: serde_json::Value = match serde_json::from_str(&content) {
        Ok(s) => s,
        Err(e) => {
            println!("cargo:warning=Failed to parse schema: {}", e);
            return;
        }
    };

    // Generate basic models based on schema analysis
    generate_models(&schema, output_dir, api_name);
}

fn generate_models(schema: &serde_json::Value, output_dir: &Path, api_name: &str) {
    let mut models = String::new();

    models.push_str("//! Generated models from OpenAPI schema\n\n");
    models.push_str("use serde::{Deserialize, Serialize};\n\n");

    // Extract schemas from components
    if let Some(components) = schema
        .get("components")
        .and_then(|c: &serde_json::Value| c.get("schemas"))
    {
        if let Some(obj) = components.as_object() {
            for (name, def) in obj {
                models.push_str(&generate_model_struct(name, def));
            }
        }
    }

    let output_path = output_dir.join(format!("{}_models.rs", api_name));
    fs::write(&output_path, models).expect("Failed to write models file");
}

fn generate_model_struct(name: &str, def: &serde_json::Value) -> String {
    let struct_name = to_pascal_case(name);
    let mut fields = String::new();

    if let Some(obj) = def
        .get("properties")
        .and_then(|p: &serde_json::Value| p.as_object())
    {
        let required: Vec<&str> = def
            .get("required")
            .and_then(|r: &serde_json::Value| r.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();

        for (prop_name, prop_def) in obj {
            let field_name = to_snake_case(prop_name);
            let field_type = get_rust_type(prop_def);
            let is_required = required.contains(&prop_name.as_ref());

            if is_required {
                fields.push_str(&format!("    pub {}: {},\n", field_name, field_type));
            } else {
                fields.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                fields.push_str(&format!(
                    "    pub {}: Option<{}>,\n",
                    field_name, field_type
                ));
            }
        }
    }

    if fields.is_empty() {
        fields.push_str("    // No properties defined\n");
    }

    format!(
        "#[derive(Serialize, Deserialize, Debug, Clone)]\npub struct {} {{\n{}}}\n\n",
        struct_name, fields
    )
}

fn get_rust_type(def: &serde_json::Value) -> String {
    if let Some(obj) = def.as_object() {
        if let Some(type_val) = obj.get("type") {
            if let Some(t) = type_val.as_str() {
                return match t {
                    "string" => "String".to_string(),
                    "integer" => "i64".to_string(),
                    "number" => "f64".to_string(),
                    "boolean" => "bool".to_string(),
                    "array" => {
                        if let Some(items) = obj.get("items") {
                            let item_type = get_rust_type(items);
                            format!("Vec<{}>", item_type)
                        } else {
                            "Vec<serde_json::Value>".to_string()
                        }
                    }
                    "object" => "serde_json::Value".to_string(),
                    _ => "serde_json::Value".to_string(),
                };
            }
        }

        // Check for $ref
        if let Some(ref_val) = obj.get("$ref").and_then(|r: &serde_json::Value| r.as_str()) {
            let name = ref_val.split('/').next_back().unwrap_or("Unknown");
            return to_pascal_case(name);
        }
    }

    "serde_json::Value".to_string()
}

fn to_pascal_case(s: &str) -> String {
    s.split(['_', '-', ' '])
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_upper = true;
        } else if c == '-' || c == ' ' {
            result.push('_');
            prev_upper = false;
        } else {
            result.push(c);
            prev_upper = false;
        }
    }

    result
}
