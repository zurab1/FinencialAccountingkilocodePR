mod models;
mod database;
mod handlers;

use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::Extension,
};
use tower::ServiceBuilder;
use tower_http::{
    services::ServeDir,
    cors::CorsLayer,
};
use std::sync::Arc;
use tracing_subscriber;

use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub database: Arc<Database>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Database setup
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:accounting.db".to_string());
    
    let db = Database::new(&database_url).await?;
    let app_state = AppState {
        database: Arc::new(db),
    };

    // Build our application with routes
    let app = Router::new()
        // Web interface routes
        .route("/", get(handlers::web::dashboard))
        .route("/accounts", get(handlers::web::accounts_page))
        .route("/transactions", get(handlers::web::transactions_page))
        .route("/reports", get(handlers::web::reports_page))
        .route("/reports/trial-balance", get(handlers::web::trial_balance_page))
        
        // API routes for accounts
        .route("/api/accounts", get(handlers::accounts::list_accounts))
        .route("/api/accounts", post(handlers::accounts::create_account))
        .route("/api/accounts/:id", get(handlers::accounts::get_account))
        .route("/api/accounts/:id", put(handlers::accounts::update_account))
        .route("/api/accounts/:id", delete(handlers::accounts::delete_account))
        
        // API routes for transactions
        .route("/api/transactions", get(handlers::transactions::list_transactions))
        .route("/api/transactions", post(handlers::transactions::create_transaction))
        .route("/api/transactions/:id", get(handlers::transactions::get_transaction))
        
        // API routes for reports
        .route("/api/reports/summary", get(handlers::reports::account_summary))
        .route("/api/reports/trial-balance", get(handlers::reports::trial_balance))
        .route("/api/reports/balance-sheet", get(handlers::reports::balance_sheet))
        .route("/api/reports/income-statement", get(handlers::reports::income_statement))
        
        // API route for transaction validation
        .route("/api/transactions/validate", post(handlers::transactions::validate_transaction))
        
        // Serve static files
        .nest_service("/static", ServeDir::new("static"))
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(Extension(app_state))
        );

    // Run the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("🚀 Financial Accounting System running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}