//! Data Transfer Objects - Request/Response types for API endpoints

pub mod auth;
pub mod user;

pub use auth::{AuthResponse, LoginRequest, RegisterRequest};
pub use user::UserResponse;
