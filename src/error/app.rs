//! App error - Top-level application error with ResponseError impl

use super::{AuthError, ErrorCode, ErrorResponse, SupabaseError};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt;
use tracing::{error, warn};

/// Main application error type - all errors bubble up to this
#[derive(Debug)]
pub enum AppError {
    Auth(AuthError),
    Validation { field: &'static str, message: String },
}

impl AppError {
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Auth(e) => e.code(),
            Self::Validation { .. } => ErrorCode::ValidationFailed,
        }
    }

    pub fn validation(field: &'static str, message: impl Into<String>) -> Self {
        Self::Validation {
            field,
            message: message.into(),
        }
    }

    /// Automatic logging based on error type
    fn log(&self) {
        match self {
            Self::Auth(AuthError::InvalidCredentials) => {
                warn!(error_code = %self.code().as_str(), "Authentication failed: invalid credentials");
            }
            Self::Auth(AuthError::External(e)) => {
                error!(error_code = %self.code().as_str(), supabase_error = %e, "External auth service error");
            }
            Self::Validation { field, message } => {
                warn!(error_code = %self.code().as_str(), field = %field, message = %message, "Validation error");
            }
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auth(e) => write!(f, "{}", e),
            Self::Validation { field, message } => {
                write!(f, "Validation error on '{}': {}", field, message)
            }
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Auth(e) => Some(e),
            Self::Validation { .. } => None,
        }
    }
}

impl From<AuthError> for AppError {
    fn from(err: AuthError) -> Self {
        Self::Auth(err)
    }
}

impl From<SupabaseError> for AppError {
    fn from(err: SupabaseError) -> Self {
        Self::Auth(AuthError::from(err))
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        self.code().status()
    }

    fn error_response(&self) -> HttpResponse {
        self.log();

        let code = self.code();
        HttpResponse::build(code.status()).json(ErrorResponse {
            code: code.as_str(),
            message: code.message(),
            field: match self {
                Self::Validation { field, .. } => Some(*field),
                _ => None,
            },
        })
    }
}

/// Convenient Result type for handlers
pub type AppResult<T> = Result<T, AppError>;
