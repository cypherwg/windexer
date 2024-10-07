use serde_json::json;
use std::fs::File;
use std::io::Write;
use windexer::storage::{Account, Transaction};

fn generate_schema<T>(name: &str) where T: schemars::JsonSchema {
    let schema = schemars::schema_for!(T);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    let mut file = File::create(format!("config/schemas/{}.json", name)).unwrap();
    file.write_all(schema_json.as_bytes()).unwrap();
    println!("Generated schema for {}", name);
}

fn main() {
    generate_schema::<Account>("account");
    generate_schema::<Transaction>("transaction");

    let combined_schema = json!({
        "definitions": {
            "Account": schemars::schema_for!(Account),
            "Transaction": schemars::schema_for!(Transaction)
        }
    });
    let combined_schema_json = serde_json::to_string_pretty(&combined_schema).unwrap();
    let mut file = File::create("config/schemas/combined.json").unwrap();
    file.write_all(combined_schema_json.as_bytes()).unwrap();
    println!("Generated combined schema");
}