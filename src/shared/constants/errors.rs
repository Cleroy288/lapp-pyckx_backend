//! Error constants - codes, messages, and HTTP status mappings.
//!
//! All error-related constants live here. The error module
//! imports these for consistent error responses across the application.

use actix_web::http::StatusCode;

// ============================================================================
// ERROR CODES - String identifiers sent to clients
// ============================================================================

pub mod codes {
    // Auth
    pub const AUTH_INVALID_CREDENTIALS: &str = "AUTH_INVALID_CREDENTIALS";

    // Supabase
    pub const SUPABASE_HTTP_ERROR: &str = "SUPABASE_HTTP_ERROR";
    pub const SUPABASE_NETWORK_ERROR: &str = "SUPABASE_NETWORK_ERROR";
    pub const SUPABASE_PARSE_ERROR: &str = "SUPABASE_PARSE_ERROR";
    pub const SUPABASE_TIMEOUT: &str = "SUPABASE_TIMEOUT";

    // Validation
    pub const VALIDATION_FAILED: &str = "VALIDATION_FAILED";
}

// ============================================================================
// ERROR MESSAGES - User-friendly messages (safe to expose to clients)
// ============================================================================

pub mod messages {
    // Auth
    pub const AUTH_INVALID_CREDENTIALS: &str = "Invalid email or password";

    // Supabase
    pub const SUPABASE_HTTP_ERROR: &str = "Authentication service error";
    pub const SUPABASE_NETWORK_ERROR: &str = "Unable to reach authentication service";
    pub const SUPABASE_PARSE_ERROR: &str = "Authentication service returned invalid data";
    pub const SUPABASE_TIMEOUT: &str = "Authentication service timed out";

    // Validation
    pub const VALIDATION_FAILED: &str = "Invalid input data";
}

// ============================================================================
// HTTP STATUS MAPPING
// ============================================================================

pub mod status {
    use super::*;

    pub const AUTH_INVALID_CREDENTIALS: StatusCode = StatusCode::UNAUTHORIZED;

    pub const SUPABASE_HTTP_ERROR: StatusCode = StatusCode::BAD_GATEWAY;
    pub const SUPABASE_NETWORK_ERROR: StatusCode = StatusCode::BAD_GATEWAY;
    pub const SUPABASE_PARSE_ERROR: StatusCode = StatusCode::BAD_GATEWAY;
    pub const SUPABASE_TIMEOUT: StatusCode = StatusCode::GATEWAY_TIMEOUT;

    pub const VALIDATION_FAILED: StatusCode = StatusCode::BAD_REQUEST;
}
