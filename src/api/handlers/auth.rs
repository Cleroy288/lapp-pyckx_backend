//! Auth handlers - HTTP endpoints for authentication

use crate::api::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::app::App;
use crate::error::{AppError, AppResult};
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpResponse, post, web};
use tracing::{info, instrument};
use validator::Validate;

// ============================================================================
// ROUTE CONFIGURATION
// ============================================================================

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login_handler)
            .service(register_handler)
            .service(logout_handler),
    );
}

// ============================================================================
// VALIDATION HELPER
// ============================================================================

fn validate_request<T: Validate>(req: &T) -> AppResult<()> {
    if let Err(errors) = req.validate() {
        for (field, field_errors) in errors.field_errors() {
            if let Some(err) = field_errors.first() {
                let message = err
                    .message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| "Validation failed".to_string());

                let static_field: &'static str = match field {
                    "email" => "email",
                    "password" => "password",
                    "username" => "username",
                    _ => "unknown",
                };

                return Err(AppError::validation(static_field, message));
            }
        }
    }
    Ok(())
}

// ============================================================================
// HANDLERS
// ============================================================================

/// POST /auth/login
#[post("/login")]
#[instrument(skip(app, req), fields(email = %req.email))]
async fn login_handler(
    app: web::Data<App>,
    req: web::Json<LoginRequest>,
) -> AppResult<HttpResponse> {
    validate_request(&req.0)?;

    let user = app.auth.login(&req.email, &req.password).await?;
    let session_id = app.auth.sessions().create_session(user.clone());

    let session_cookie = Cookie::build("session_id", session_id.clone())
        .http_only(true)
        .secure(app.config.secure_http.parse().unwrap()) // Set true in production with HTTPS via .env
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    let response = AuthResponse::from_user(&user);

    info!(session_id = %session_id, "Login successful");
    Ok(HttpResponse::Ok().cookie(session_cookie).json(response))
}

/// POST /auth/register
#[post("/register")]
#[instrument(skip(app, req), fields(email = %req.email, username = %req.username))]
async fn register_handler(
    app: web::Data<App>,
    req: web::Json<RegisterRequest>,
) -> AppResult<HttpResponse> {
    validate_request(&req.0)?;

    let user = app
        .auth
        .register(
            &req.email,
            &req.password,
            &req.username,
            req.phone_country_code.as_deref(),
            req.phone_number.as_deref(),
        )
        .await?;

    let session_id = app.auth.sessions().create_session(user.clone());

    let session_cookie = Cookie::build("session_id", session_id.clone())
        .http_only(true)
        .secure(app.config.secure_http.parse().unwrap())
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    let response = AuthResponse::from_user(&user);

    info!(session_id = %session_id, "Registration successful");
    Ok(HttpResponse::Created()
        .cookie(session_cookie)
        .json(response))
}

/// POST /auth/logout
#[post("/logout")]
#[instrument(skip(app, req))]
async fn logout_handler(app: web::Data<App>, req: actix_web::HttpRequest) -> HttpResponse {
    let session_id = extract_session_id(&req);

    if let Some(ref sid) = session_id {
        app.auth.logout(sid).await;
    } else {
        info!("Logout called without session cookie");
    }

    let session_cookie = Cookie::build("session_id", "")
        .http_only(true)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    HttpResponse::Ok()
        .cookie(session_cookie)
        .json(serde_json::json!({"message": "Logged out successfully"}))
}

// ============================================================================
// HELPERS
// ============================================================================

fn extract_session_id(req: &actix_web::HttpRequest) -> Option<String> {
    let cookies = req.cookies().ok()?;
    cookies
        .iter()
        .find(|c| c.name() == "session_id")
        .map(|c| c.value().to_string())
}
