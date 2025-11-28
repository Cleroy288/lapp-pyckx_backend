//! Infrastructure layer - External service integrations.
//!
//! This layer handles all communication with external services like databases,
//! third-party APIs, message queues, etc. It translates between external
//! formats and domain types.

pub mod supabase;

pub use supabase::SupabaseClient;
