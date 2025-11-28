//! Auth DTOs - Request/Response types for authentication endpoints

use crate::domain::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

// ============================================================================
// REQUEST DTOs WITH VALIDATION
// ============================================================================

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email too long"))]
    pub email: String,

    #[validate(length(min = 6, max = 128, message = "Password must be 6-128 characters"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(email(message = "Invalid email format"))]
    #[validate(length(max = 255, message = "Email too long"))]
    pub email: String,

    #[validate(length(min = 6, max = 128, message = "Password must be 6-128 characters"))]
    pub password: String,

    #[validate(length(min = 3, max = 50, message = "Username must be 3-50 characters"))]
    pub username: String,

    #[validate(length(max = 5, message = "Country code too long"))]
    pub phone_country_code: Option<String>,

    #[validate(length(max = 20, message = "Phone number too long"))]
    pub phone_number: Option<String>,
}

// ============================================================================
// RESPONSE DTO - Only safe data sent to frontend (no tokens!)
// ============================================================================

#[derive(Serialize)]
pub struct AuthResponse {
    pub username: String,
    pub email: String,
    pub role: String,
}

impl AuthResponse {
    pub fn from_user(u: &User) -> Self {
        Self {
            username: u.username.clone(),
            email: u.email.clone(),
            role: u.role.clone(),
        }
    }
}
