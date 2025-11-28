//! User handlers - HTTP endpoints for user operations

use crate::api::dto::UserResponse;
use crate::app::App;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

// ============================================================================
// ROUTE CONFIGURATION
// ============================================================================

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(me_handler));
}

// ============================================================================
// HANDLERS
// ============================================================================

/// GET /user/me - Get current user from session
#[get("/me")]
async fn me_handler(app: web::Data<App>, req: HttpRequest) -> impl Responder {
    let session_id = match extract_session_id(&req) {
        Some(id) => id,
        None => return HttpResponse::Unauthorized().finish(),
    };

    match app.auth.sessions().get_user(&session_id) {
        Some(user) => HttpResponse::Ok().json(UserResponse::from(&user)),
        None => HttpResponse::Unauthorized().finish(),
    }
}

// ============================================================================
// HELPERS
// ============================================================================

fn extract_session_id(req: &HttpRequest) -> Option<String> {
    let cookies = req.cookies().ok()?;
    cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .map(|c| c.value().to_string())
}
