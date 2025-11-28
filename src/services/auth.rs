//! Authentication service - Orchestrates login, register, logout flows

use crate::config::Config;
use crate::domain::{SessionStore, User};
use crate::error::{AppError, AppResult, AuthError};
use crate::infrastructure::SupabaseClient;
use std::fmt;
use tracing::{info, instrument};

/// Authentication service - coordinates auth flows
#[derive(Clone, Debug)]
pub struct AuthService {
    supabase: SupabaseClient,
    sessions: SessionStore,
}

impl AuthService {
    pub fn new(cfg: &Config, sessions: SessionStore) -> Self {
        info!("AuthService initialized");
        Self {
            supabase: SupabaseClient::new(cfg),
            sessions,
        }
    }

    /// Login user with email and password
    /// Returns User on success, AppError on failure (automatically logged)
    #[instrument(skip(self, password), fields(email = %email))]
    pub async fn login(&self, email: &str, password: &str) -> AppResult<User> {
        let user = self
            .supabase
            .login(email, password)
            .await
            .map_err(|e| AppError::Auth(AuthError::from(e)))?;

        self.sessions.create_session(user.clone());
        info!(user_id = %user.id, "User logged in");

        Ok(user)
    }

    /// Register a new user with profile data
    #[instrument(skip(self, password), fields(email = %email, username = %username))]
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        username: &str,
        phone_country_code: Option<&str>,
        phone_number: Option<&str>,
    ) -> AppResult<User> {
        let user = self
            .supabase
            .register(email, password, username, phone_country_code, phone_number)
            .await
            .map_err(|e| AppError::Auth(AuthError::from(e)))?;

        self.sessions.create_session(user.clone());
        info!(user_id = %user.id, "User registered");

        Ok(user)
    }

    /// Logout user - invalidates session locally and notifies Supabase
    #[instrument(skip(self, session_id))]
    pub async fn logout(&self, session_id: &str) -> bool {
        // Remove from local store and get user data (contains access_token)
        let user = self.sessions.delete_session(session_id);

        if let Some(user) = user {
            // Notify Supabase to invalidate the token (best-effort)
            self.supabase.logout(&user.access_token).await;
            info!(session_id = %session_id, "User logged out");
            true
        } else {
            info!(session_id = %session_id, "Logout attempted but session not found");
            false
        }
    }

    /// Get session store reference (for user lookups)
    pub fn sessions(&self) -> &SessionStore {
        &self.sessions
    }
}

impl fmt::Display for AuthService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AuthService({})", self.supabase)
    }
}
