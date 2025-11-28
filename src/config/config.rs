//! Application configuration - Environment variables

use dotenv::dotenv;
use std::env;
use std::fmt;
use tracing::info;

/// Application configuration loaded from environment
#[derive(Debug, Clone)]
pub struct Config {
    pub ip: String,
    pub port: String,
    #[allow(dead_code)]
    pub sp_id: String,
    pub sp_url: String,
    pub sp_anon: String,
    #[allow(dead_code)]
    pub sp_service_role: String,
    pub secure_http: String,
}

impl Config {
    const DEF_ERR: &'static str = "must be set in .env";

    pub fn from_env() -> Self {
        dotenv().ok();

        let config = Self {
            ip: env::var("IP").unwrap_or_else(|_| panic!("IP {}", Self::DEF_ERR)),
            port: env::var("PORT").unwrap_or_else(|_| panic!("PORT {}", Self::DEF_ERR)),
            sp_id: env::var("SP_ID").unwrap_or_else(|_| panic!("SP_ID {}", Self::DEF_ERR)),
            sp_url: env::var("SP_URL").unwrap_or_else(|_| panic!("SP_URL {}", Self::DEF_ERR)),
            sp_anon: env::var("SP_ANON").unwrap_or_else(|_| panic!("SP_ANON {}", Self::DEF_ERR)),
            sp_service_role: env::var("SP_SERVICE_ROLE")
                .unwrap_or_else(|_| panic!("SP_SERVICE_ROLE {}", Self::DEF_ERR)),
            secure_http: env::var("SECURE_HTTP")
                .unwrap_or_else(|_| panic!("SECURE_HTTP {}", Self::DEF_ERR)),
        };

        info!(
            ip = %config.ip,
            port = %config.port,
            supabase_url = %config.sp_url,
            "Configuration loaded"
        );

        config
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config({}:{})", self.ip, self.port)
    }
}
