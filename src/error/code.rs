//! Error codes - maps enum variants to constants

use crate::shared::constants::errors::{codes, messages, status};
use actix_web::http::StatusCode;

/// Centralized error codes for consistent API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    // Auth
    InvalidCredentials,
    // Supabase
    SupabaseHttpError,
    SupabaseNetworkError,
    SupabaseParseError,
    SupabaseTimeout,
    // Validation
    ValidationFailed,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => codes::AUTH_INVALID_CREDENTIALS,
            Self::SupabaseHttpError => codes::SUPABASE_HTTP_ERROR,
            Self::SupabaseNetworkError => codes::SUPABASE_NETWORK_ERROR,
            Self::SupabaseParseError => codes::SUPABASE_PARSE_ERROR,
            Self::SupabaseTimeout => codes::SUPABASE_TIMEOUT,
            Self::ValidationFailed => codes::VALIDATION_FAILED,
        }
    }

    pub fn message(&self) -> &'static str {
        match self {
            Self::InvalidCredentials => messages::AUTH_INVALID_CREDENTIALS,
            Self::SupabaseHttpError => messages::SUPABASE_HTTP_ERROR,
            Self::SupabaseNetworkError => messages::SUPABASE_NETWORK_ERROR,
            Self::SupabaseParseError => messages::SUPABASE_PARSE_ERROR,
            Self::SupabaseTimeout => messages::SUPABASE_TIMEOUT,
            Self::ValidationFailed => messages::VALIDATION_FAILED,
        }
    }

    pub fn status(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => status::AUTH_INVALID_CREDENTIALS,
            Self::SupabaseHttpError => status::SUPABASE_HTTP_ERROR,
            Self::SupabaseNetworkError => status::SUPABASE_NETWORK_ERROR,
            Self::SupabaseParseError => status::SUPABASE_PARSE_ERROR,
            Self::SupabaseTimeout => status::SUPABASE_TIMEOUT,
            Self::ValidationFailed => status::VALIDATION_FAILED,
        }
    }
}
