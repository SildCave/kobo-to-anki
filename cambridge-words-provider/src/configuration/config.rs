use anyhow::Result;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub metrics_server: MetricsServerConfig,
    pub postgres_database: PostgresDatabaseConfig,
    pub endpoint_rate_limiters: EndpointRateLimitersConfig,
    pub cambrinary_session_tracker: CambrinarySessionTrackerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_https: bool,
    pub allow_non_cloudflare_ips: bool,
    pub cloudflare_ips_refresh_interval_s: Option<u64>,
    pub cloudflare_ips_refresh_interval_jitter_s: Option<u64>,
    pub pem_cert_path: Option<String>,
    pub pem_key_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetricsServerConfig {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PostgresDatabaseConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EndpointRateLimitersConfig {
    pub get_word: RateLimiterConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimiterConfig {
    pub burst_size: u32,
    pub max_per_second: u64,
    pub burst_size_global: u32,
    pub max_per_second_global: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CambrinarySessionTrackerConfig {
    pub max_sessions: u32,
    pub session_acquire_cooldown_ms: u64,
    pub session_acquire_cooldown_jitter_ms: u64,
}

impl Config {
    pub fn from_file(
        path: impl Into<std::path::PathBuf>,
    ) -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(path.into().to_str().unwrap()))
            .build()?;
        let config: Config = settings.try_deserialize()?;

        Ok(config)
    }
}