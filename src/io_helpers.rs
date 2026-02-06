use serde_json::{Value, json};
use std::io::{self, Read};

pub fn print_json(value: &Value) {
    let rendered = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string());
    println!("{rendered}");
}

pub fn print_error(error_code: &str, message: impl AsRef<str>) {
    print_json(&json!({
        "status": "error",
        "error_code": error_code,
        "message": message.as_ref()
    }));
}

pub fn read_stdin_json() -> anyhow::Result<Value> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    if input.trim().is_empty() {
        anyhow::bail!("Expected JSON input via stdin");
    }
    let parsed: Value = serde_json::from_str(&input)?;
    Ok(parsed)
}

pub fn home_dir() -> anyhow::Result<std::path::PathBuf> {
    std::env::var_os("HOME")
        .map(std::path::PathBuf::from)
        .ok_or_else(|| anyhow::anyhow!("Unable to determine HOME directory"))
}
