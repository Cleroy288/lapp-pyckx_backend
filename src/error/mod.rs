//! Centralized error handling with automatic tracing integration.
//!
//! Error flow: SupabaseError → AuthError → AppError → HTTP Response
//! Each layer adds context and all errors are automatically logged.
//!
//! # Module Structure
//! - `code.rs` - ErrorCode enum mapping to constants
//! - `supabase.rs` - Infrastructure layer errors
//! - `auth.rs` - Service layer errors
//! - `app.rs` - Application layer errors + ResponseError
//! - `response.rs` - JSON error response structure

mod app;
mod auth;
mod code;
mod response;
mod supabase;

// Re-export all public types
pub use app::{AppError, AppResult};
pub use auth::AuthError;
pub use code::ErrorCode;
pub use response::ErrorResponse;
pub use supabase::SupabaseError;
