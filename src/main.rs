// ============================================================================
// LAPP - Login Application
// ============================================================================

// Modules
mod api;
mod app;
mod apps;
mod config;
mod domain;
mod error;
mod infrastructure;
mod services;
mod shared;

#[cfg(test)]
mod tests;

// Imports
use actix_web::{middleware::Logger, rt::signal, web, App as ActixApp, HttpServer};
use app::App;
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// ============================================================================
// MAIN
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    init_tracing();

    // Initialize application
    let app = App::new();
    info!(
        name = %app.name,
        version = %app.version,
        ip = %app.config.ip,
        port = %app.config.port,
        "Starting server"
    );

    // Share app state across handlers
    let app_data = web::Data::new(app.clone());

    // Build and run server
    let server = HttpServer::new(move || {
        ActixApp::new()
            .app_data(app_data.clone())
            // Request logging middleware
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            // Configure routes
            .configure(api::init)
    })
    .bind((
        app.config.ip.clone(),
        app.config.port.parse::<u16>().expect("Invalid port"),
    ))?
    .run();

    // Graceful shutdown on Ctrl+C
    let srv_handle = server.handle();
    tokio::spawn(async move {
        signal::ctrl_c().await.unwrap();
        info!("Received CTRL+C, shutting down gracefully...");
        srv_handle.stop(true).await;
    });

    server.await
}

// ============================================================================
// TRACING SETUP
// ============================================================================

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,actix_web=info,actix_server=info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false)
                .compact(),
        )
        .init();
}
