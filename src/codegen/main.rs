use std::env;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: codegen <openapi_schema.json> [output_dir]");
        eprintln!();
        eprintln!("Arguments:");
        eprintln!("  openapi_schema.json  Path to the OpenAPI schema file");
        eprintln!("  output_dir           Optional output directory (default: src/generated)");
        eprintln!();
        eprintln!("Note: The build.rs script automatically generates code from");
        eprintln!("forum.json and market.json during compilation.");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let output_dir = args
        .get(2)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("src/generated"));

    if !input_path.exists() {
        eprintln!("Error: Input file does not exist: {:?}", input_path);
        std::process::exit(1);
    }

    println!("OpenAPI Schema: {:?}", input_path);
    println!("Output Directory: {:?}", output_dir);
    println!();
    println!("Code generation is handled by the build.rs script during compilation.");
    println!("To generate code from a custom schema:");
    println!("  1. Copy your schema to the project root");
    println!("  2. Update build.rs to include your schema");
    println!("  3. Run: cargo build");
    println!();
    println!("Alternatively, the generated files in src/generated/ can be used directly.");
}
