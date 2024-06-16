use std::path::Path;

use serde::Deserialize;
use tokio::fs::read_to_string;

pub const COUNTRIES_CSV: &[u8] = include_bytes!("../countries.csv");

#[derive(Debug, Deserialize)]
pub struct Config {
    pub consumer: ConfigKeySecret,
    pub access: ConfigKeySecret,
}

#[derive(Debug, Deserialize)]
pub struct ConfigKeySecret {
    pub key: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub name: String,
    pub emoji: String,
}

pub fn parse_csv() -> Option<Vec<Country>> {
    let mut builder = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .from_reader(COUNTRIES_CSV);

    builder
        .deserialize()
        .map(|x| x.ok())
        .collect::<Option<Vec<Country>>>()
}

pub async fn parse_config(path: &Path) -> Result<Config, String> {
    if !path.is_file() {
        return Err("Path is not a file".to_string());
    }

    let out = read_to_string(path)
        .await
        .map_err(|x| format!("Failed to read data to string: {x}"))?;

    let parsed =
        toml::from_str::<Config>(&out).map_err(|x| format!("Failed to parse data: {x}"))?;

    Ok(parsed)
}
