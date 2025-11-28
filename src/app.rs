//! Application state - Main application struct holding all services

use crate::apps::CollectionApp;
use crate::config::Config;
use crate::domain::{AppModule, SessionStore};
use crate::services::AuthService;
use tracing::info;

/// Main application struct - holds all services and shared state
#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub version: String,
    pub config: Config,
    pub auth: AuthService,
    // Apps
    pub collection: CollectionApp,
}

impl App {
    pub fn new() -> Self {
        let cfg = Config::from_env();
        let sessions = SessionStore::new();
        let auth = AuthService::new(&cfg, sessions);
        let collection = CollectionApp::new();

        info!(
            collection_app = %collection.name(),
            "Apps initialized"
        );

        Self {
            name: "LAPP".to_string(),
            version: "0.1.0".to_string(),
            config: cfg,
            auth,
            collection,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
