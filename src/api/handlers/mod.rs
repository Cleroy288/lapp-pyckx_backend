//! HTTP handlers - Route handlers organized by feature

pub mod auth;
pub mod user;

use actix_web::web;

/// Initialize all API routes
pub fn init(cfg: &mut web::ServiceConfig) {
    auth::init(cfg);
    user::init(cfg);
}
