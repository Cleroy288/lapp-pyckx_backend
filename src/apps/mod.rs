//! Apps module - Contains all app implementations
//!
//! Each app is a self-contained feature module with its own
//! domain entities, services, and API handlers.

pub mod collection;

pub use collection::CollectionApp;
