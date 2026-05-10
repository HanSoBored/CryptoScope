//! CryptoScope API Server
//!
//! Axum-based REST API with OpenAPI documentation, authentication, rate limiting, and CORS.

use anyhow::Context;
use axum::Router;
use axum::http::{HeaderValue, Method, header};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod core;

use api::AppState;

/// Initialize tracing subscriber with environment-based filtering
fn init_logging() -> anyhow::Result<()> {
    // Load .env file (if exists)
    dotenvy::dotenv().ok();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,cryptoscope=debug"));
    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_thread_ids(false)
                .with_line_number(false),
        )
        .init();
    Ok(())
}

/// Pre-initialize database schema (tolerant of errors for idempotent startup)
fn pre_init_db_schema(db_path: &str) {
    match core::db::create_connection() {
        Ok(conn) => {
            if let Err(e) = core::db::init_schema(&conn) {
                info!(
                    "Database schema already exists or initialization skipped: {}",
                    e
                );
            } else {
                info!("Database schema initialized at {}", db_path);
            }
        }
        Err(e) => {
            info!("Database will be initialized on first use: {}", e);
        }
    }
}

/// Resolve and validate DATABASE_PATH from environment variable
fn resolve_db_path() -> anyhow::Result<String> {
    let database_path =
        std::env::var("DATABASE_PATH").unwrap_or_else(|_| "./cryptoscope_data".to_string());
    let db_path = core::utils::path::validate_and_normalize_path(&database_path)
        .map_err(|e| anyhow::anyhow!("Invalid DATABASE_PATH: {e}"))?;
    Ok(db_path.to_string_lossy().to_string())
}

/// OpenAPI documentation for the CryptoScope API
#[derive(OpenApi)]
#[openapi(
    paths(
        api::health_check,
        api::exchanges::get_exchanges,
        api::symbols::get_symbols,
        api::screener::run_screener,
        api::stats::get_stats,
        api::refresh::refresh_cache,
        api::auth::login,
    ),
    components(
        schemas(
            api::types::ExchangeListResponse,
            api::types::SymbolResponse,
            api::types::ScreenerResponse,
            api::types::StatsResponse,
            api::types::RefreshResponse,
            api::types::ErrorResponse,
            api::types::ValidationErrorResponse,
            api::types::ScreenerModeQuery,
            api::auth::LoginRequest,
            api::auth::LoginResponse,
            crate::core::models::Symbol,
            crate::core::models::Statistics,
            crate::core::models::PriceChange,
            crate::core::models::ContractType,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Exchanges", description = "Exchange management"),
        (name = "Symbols", description = "Symbol/instrument queries"),
        (name = "Screener", description = "Price screener and analysis"),
        (name = "Statistics", description = "Market statistics"),
        (name = "Cache", description = "Cache management"),
        (name = "Authentication", description = "JWT authentication"),
    ),
    info(
        title = "CryptoScope API",
        version = "0.5.0",
        description = "Multi-exchange crypto symbols intelligence API"
    ),
    security(
        ("bearer_auth" = []),
    )
)]
struct ApiDoc;

/// Build the application router with all routes and middleware
fn build_app(state: AppState, openapi: utoipa::openapi::OpenApi) -> Router {
    // Use api::router() as single source of truth for routes and rate limiting
    let app = api::router()
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer());

    Router::new()
        .merge(SwaggerUi::new("/api-docs/swagger-ui").url("/api-docs/openapi.json", openapi))
        .merge(app)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_logging()?;

    info!(
        "Starting CryptoScope API server v{}...",
        env!("CARGO_PKG_VERSION")
    );

    // Validate and sanitize database path
    let db_path = resolve_db_path()?;

    // Initialize database schema
    pre_init_db_schema(&db_path);

    // Load JWT keys and admin credentials with proper error handling
    let keys =
        api::auth::load_keys().map_err(|e| anyhow::anyhow!("Failed to load JWT keys: {e}"))?;
    let admin_credentials = api::auth::load_admin_credentials()
        .map_err(|e| anyhow::anyhow!("Failed to load admin credentials: {e}"))?;

    // Create application state
    let state = AppState {
        keys,
        admin_credentials,
    };

    // Generate OpenAPI spec
    let openapi = ApiDoc::openapi();

    // Build application
    let app = build_app(state, openapi);

    // Get the port from environment or use default
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    info!("Server listening on {}", addr);
    info!(
        "OpenAPI docs available at http://{}/api-docs/swagger-ui",
        addr
    );
    info!(
        "OpenAPI JSON available at http://{}/api-docs/openapi.json",
        addr
    );

    // Start the server with connect info for IP-based rate limiting
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .with_context(|| format!("Failed to bind to {addr}"));

    axum::serve(
        listener?,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;

    Ok(())
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Shutting down gracefully...");
}

/// Create CORS layer for cross-origin requests
///
/// Configuration is environment-based:
/// - Production: Explicit allowlist from CORS_ORIGINS
/// - Development: localhost variants
fn cors_layer() -> CorsLayer {
    // Check for permissive mode (development only)
    if std::env::var("CORS_PERMISSIVE").is_ok() {
        let is_production = std::env::var("RUST_ENV")
            .map(|e| e == "production" || e == "prod")
            .unwrap_or(false);

        if is_production {
            // Graceful fallback: log error and use restrictive CORS instead of panicking
            tracing::error!(
                "CORS_PERMISSIVE is set in production - this is a security risk. \
                 Falling back to restrictive CORS (localhost only). \
                 Set CORS_ORIGINS explicitly for production use."
            );
            return restrictive_cors();
        }

        tracing::warn!("CORS_PERMISSIVE is set - allowing all origins (development only)");
        return CorsLayer::very_permissive();
    }

    // Parse allowed origins from environment
    let origins: Vec<HeaderValue> = std::env::var("CORS_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000,http://localhost:3001".into())
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    if origins.is_empty() {
        tracing::warn!("No valid CORS origins configured, using localhost only");
        return restrictive_cors();
    }

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .expose_headers([header::CONTENT_TYPE, header::WWW_AUTHENTICATE])
}

/// Create a restrictive CORS layer for localhost-only access.
///
/// Used as a safe default when no CORS origins are configured or when
/// permissive mode is incorrectly set in production.
fn restrictive_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .expose_headers([header::CONTENT_TYPE, header::WWW_AUTHENTICATE])
}
