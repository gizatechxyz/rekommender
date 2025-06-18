//! API Endpoints
//! 
//! - `GET /health` - Service health check
//! - `POST /process` - Process articles and generate similarity matrix
//! - `GET /download/<request_id>` - Download processing results as ZIP
//! - `POST /cleanup` - Manually trigger cleanup of old results

#[macro_use]
extern crate rocket;

use std::fs;

use rocket::routes;
use rocket::catchers;

mod api;
mod config;
mod recommender;
mod types;
mod utils;

use api::{CleanupFairing, download_result, health, manual_cleanup, process_articles};
use api::{internal_error, not_found, unauthorized};
use config::{AppConfig, AppState};

/// Initialize and launch the Rocket web server
/// 
/// Loads configuration from environment variables, sets up the application state,
/// configures routes and error handlers, and starts background cleanup tasks.
/// 
/// # Environment Variables
/// 
/// - `PORT` - Server port (default: 8080)
/// - `MAX_UPLOAD_SIZE_MB` - Maximum upload size in MB (default: 100)
/// - `API_KEY` - Required API key for authentication (minimum 16 characters)
/// - `OUTPUT_DIR` - Directory for storing results (default: ./outputs)
/// - `RESULT_TTL_HOURS` - Time-to-live for results in hours (default: 24)
/// 
/// # Panics
/// 
/// Panics if:
/// - Required environment variables are missing or invalid
/// - API key is less than 16 characters
/// - Output directory cannot be created
#[launch]
fn rocket() -> _ {
    // Load configuration from environment
    let config = AppConfig::from_env();

    // Create output base directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&config.output_base_dir) {
        panic!("Failed to create output base directory: {}", e);
    }

    println!("🚀 Starting REKT Recommender API v{}", env!("CARGO_PKG_VERSION"));
    println!("📁 Output directory: {:?}", config.output_base_dir);
    println!("🔑 API authentication enabled");
    println!("🧹 Automatic cleanup: {} hours TTL", config.result_ttl_hours);

    // Build and configure Rocket instance
    rocket::build()
        .configure(rocket::Config {
            port: config.port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .attach(CleanupFairing::new(
            config.output_base_dir.clone(), 
            config.result_ttl_hours
        ))
        .manage(AppState {
            max_upload_size: config.max_upload_size,
            api_key: config.api_key,
            output_base_dir: config.output_base_dir,
            result_ttl_hours: config.result_ttl_hours,
        })
        .mount("/", routes![
            health, 
            process_articles, 
            download_result, 
            manual_cleanup
        ])
        .register("/", catchers![
            unauthorized, 
            not_found, 
            internal_error
        ])
}
