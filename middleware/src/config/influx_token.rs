use anyhow::Result;
use serde_json::Value;
use std::fs;

pub fn load_influx_token() -> Result<String> {
    let contents = fs::read_to_string("/run/influx-data/influx-token.json")?;
    let json: Value = serde_json::from_str(&contents)?;

    Ok(json["token"].as_str().unwrap().to_string())
}
