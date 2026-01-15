use crate::config::ServerConfig;
use crate::handler::{file_handler, AppState};
use anyhow::Result;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

/// Start the file sharing server
pub async fn start_server(config: ServerConfig) -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let root_path = PathBuf::from(&config.path).canonicalize()?;
    info!("Serving directory: {}", root_path.display());

    let state = AppState { root_path };

    // Build router
    let mut app = Router::new()
        .fallback(file_handler)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Add CORS if enabled
    if config.enable_cors {
        app = app.layer(CorsLayer::permissive());
    }

    // Create socket address
    let addr = format!("{}:{}", config.host, config.port);
    let socket_addr: SocketAddr = addr.parse()?;

    info!("Server listening on http://{}", socket_addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
