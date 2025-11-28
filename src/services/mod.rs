//! Services layer - Business logic orchestration.
//!
//! Services coordinate between domain entities and infrastructure.
//! They contain the core business rules and workflows.

mod auth;

pub use auth::AuthService;
