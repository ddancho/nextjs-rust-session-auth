use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u32,
    pub database_url: String,
    pub signin_key: String,
    pub session_cookie_name: String,
    pub session_ttl_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Config, config::ConfigError> {
        dotenv().ok();

        let c = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        c.try_deserialize()
    }
}
