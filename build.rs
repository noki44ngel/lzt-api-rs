//! Build script - generates common models from OpenAPI

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=forum.json");
    println!("cargo:rerun-if-changed=market.json");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let generated_dir = PathBuf::from(&manifest_dir).join("src").join("generated");
    fs::create_dir_all(&generated_dir).expect("Failed to create directory");

    // Write mod.rs
    let mod_code = r#"pub mod forum;
pub mod market;
pub mod models;

pub use forum::*;
pub use market::*;
pub use models::*;

pub use crate::client::{ForumClient, MarketClient};
"#;
    fs::write(generated_dir.join("mod.rs"), mod_code).expect("Failed to write mod.rs");

    println!("cargo:warning=Build script completed!");
}
