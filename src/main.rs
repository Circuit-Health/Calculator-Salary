// Import necessary modules and crates
use axum::{
    routing::post,
    routing::get,
    Router,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use log::{info, error};

// Module imports for additional functionality
mod config;
use config::Config;
mod health;
mod metrics;
mod tax;


#[tokio::main]
async fn main()
{
    // Initialize the logger
    env_logger::init();

    // Initialize a counter metric
    let _counter = match metrics::initialize_counter() {
        Ok(counter) => counter,
        Err(error) => {
            error!("{}", error);
            std::process::exit(1);
        }
    };

    // Load configuration
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return;
        }
    };

    // Log server start-up
    info!("Starting server at {}", config.server_address);

    // Build our application with multiple routes
    let app = Router::new()
        .route("/calculate_tax", post(calculate_tax))
        .route("/metrics", get(metrics::metrics_handler))
        .route("/health", get(health::health_check));

    // Run our app with hyper, listening globally
    // Use configuration for setting up the server
    
    match tokio::net::TcpListener::bind(&config.server_address).await {
        Ok(listener) => {
            match axum::serve(listener, app).await {
                Ok(_) => info!("Server running at {}", config.server_address),
                Err(e) => error!("Server error: {}", e),
            }
        }
        Err(e) => error!("Failed to bind to {}: {}", config.server_address, e),
    }

}

// Struct to deserialize incoming JSON data for salary input
#[derive(Deserialize)]
struct SalaryInput {
    salary: f64,
    year: u32,
    calculate_beyond_max: bool, // Field to allow calculation beyond max superannuation limit
}

// Struct to serialize data to be sent as JSON response
#[derive(Serialize)]
struct TaxOutput {
    annual_post_tax_salary: i64, // Field for annual post-tax salary
    superannuation: i64,         // Field for superannuation amount
}

// Handler function for the /calculate_tax route
async fn calculate_tax(Json(payload): Json<SalaryInput>) -> Result<Json<TaxOutput>, (StatusCode, String)> {
    if payload.salary < 0.0 {
        error!("Negative salary input: {}", payload.salary);
        return Err((StatusCode::BAD_REQUEST, "Invalid salary input".to_string()));
    }

    // Calculate tax using the tax module
    let annual_tax = tax::calculate_tax(payload.salary, payload.year);

    // Calculate superannuation
    let superannuation_rate = tax::get_superannuation_rate(payload.year);
    let mut superannuation_amount = payload.salary * (superannuation_rate / 100.0);
    
    // Cap superannuation amount at $27,500 unless specified otherwise by the user
    if !payload.calculate_beyond_max && superannuation_amount > 27500.0 {
        superannuation_amount = 27500.0;
    }

    // Prepare the output
    let annual_post_tax_salary = (payload.salary - annual_tax) as i64;
    let superannuation = superannuation_amount as i64;

    // Send the response
    Ok(Json(TaxOutput { annual_post_tax_salary, superannuation }))
}
