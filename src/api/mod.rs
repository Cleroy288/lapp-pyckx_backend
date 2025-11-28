//! API layer - HTTP interface for the application.
//!
//! This layer handles all HTTP concerns: routing, request parsing,
//! response formatting, and middleware.
//!
//! # Structure
//! - `handlers/` - Route handlers organized by feature
//! - `dto/` - Request/Response data transfer objects
//! - `extractors/` - Custom Actix extractors (future)
//! - `middleware/` - Custom middleware (future)

pub mod dto;
pub mod handlers;

pub use handlers::init;
