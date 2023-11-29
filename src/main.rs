// Import necessary modules and crates
use axum::{
    routing::post,
    Router,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use log::{info, error};
use dotenv::dotenv;
use std::env;

// Include the tax module (make sure this module exists and is correctly implemented)
mod tax;

// Main entry point for the application
#[tokio::main]
async fn main() {
    dotenv().ok(); // Load environment variables if available
    env_logger::init(); // Initialize the logger

    // Set up the web application and its routes
    let app = Router::new().route("/calculate_tax", post(calculate_tax));

    // Configure the address and port for the server
    let address = format!("{}:{}", 
        env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()), 
        env::var("PORT").unwrap_or_else(|_| "3000".to_string()));

    let addr: SocketAddr = address.parse().expect("Invalid address");
    info!("Listening on {}", addr);

    // Run the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
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
