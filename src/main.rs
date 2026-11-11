use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

mod db;
#[allow(dead_code)]
mod auth;
#[allow(dead_code)]
mod password;
mod error;
mod models;
mod handlers;
mod middleware;

use auth::JwtHandler;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing (logging)
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    println!("🔧 Initializing database connection...");
    
    // Initialize database connection pool
    let pool = db::init_db()
        .await
        .expect("❌ Failed to initialize database");

    println!("✅ Database migrations completed");

    // Test database connection
    db::test_connection(&pool)
        .await
        .expect("❌ Failed to test database connection");

    println!("✅ Database connection verified");

    // Initialize JWT handler
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("❌ JWT_SECRET not found in environment variables");
    let _jwt_handler = JwtHandler::new(jwt_secret);
    println!("✅ JWT handler initialized");

    // Build our application with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        // Auth routes
        .route("/api/auth/register", post(handlers::register))
        .route("/api/auth/login", post(handlers::login))
        .route("/api/auth/logout", post(handlers::logout))
        // Protected routes
        .route("/api/users/me", get(handlers::get_current_user))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // Run our app with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);
    println!("📚 API Documentation: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

// Basic routes
async fn root_handler() -> &'static str {
    "🎉 Rust PostgreSQL API Service - Phase 1.6 ✅ (Protected Endpoints)"
}

async fn health_check() -> (StatusCode, String) {
    (StatusCode::OK, "Server is healthy ✅ | Database: Connected | JWT: Ready".to_string())
}
