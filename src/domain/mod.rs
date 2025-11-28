//! Domain layer - Pure business entities with no external dependencies.
//!
//! This layer contains the core data structures that represent your business domain.
//! These types should be framework-agnostic and contain no HTTP, database, or external service logic.

mod app_instance;
mod session;
mod user;

pub use app_instance::{AppId, AppInstance, AppModule};
pub use session::SessionStore;
pub use user::{User, UserId};
