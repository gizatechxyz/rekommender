use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::{Context, Result};
use rocket::data::{Data, ToByteUnit};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post, catch, Rocket, State, Build};
use tokio::time::interval;
use uuid::Uuid;

use crate::config::{AppState, Config};
use crate::types::{ApiResponse, ContentJson, HealthResponse, ProcessingResult};
use crate::utils::{cleanup_old_results, create_zip_archive, save_outputs};
use crate::recommender::RecommenderSystem;

/// API key authentication guard
pub struct ApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = request.guard::<&State<AppState>>().await;

        match state {
            Outcome::Success(app_state) => {
                // Check X-API-Key header
                if let Some(key) = request.headers().get_one("X-API-Key") {
                    if key == app_state.api_key {
                        return Outcome::Success(ApiKey);
                    }
                }
                
                // Check Authorization: Bearer token format
                if let Some(key) = request.headers().get_one("Authorization") {
                    if let Some(bearer_key) = key.strip_prefix("Bearer ") {
                        if bearer_key == app_state.api_key {
                            return Outcome::Success(ApiKey);
                        }
                    }
                }
                
                Outcome::Error((Status::Unauthorized, ()))
            }
            _ => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}

/// Health check endpoint
/// 
/// Returns the current status and version of the API service
#[get("/health")]
pub fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "rekt-recommender-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Process articles from JSON input
/// 
/// Accepts a JSON payload containing articles and processes them through the
/// recommender system to generate similarity matrices and zero-knowledge proofs.
#[post("/process", data = "<data>")]
pub async fn process_articles(
    data: Data<'_>,
    state: &State<AppState>,
    content_type: &ContentType,
    _api_key: ApiKey,
) -> Result<Json<ApiResponse>, Custom<Json<ApiResponse>>> {
    let request_id = Uuid::new_v4().to_string();

    // Verify content type is JSON
    if !content_type.is_json() {
        return Err(Custom(
            Status::BadRequest,
            Json(ApiResponse {
                request_id,
                status: "error".to_string(),
                message: "Content-Type must be application/json".to_string(),
                data: None,
            }),
        ));
    }

    // Read the uploaded JSON data
    let bytes = match data.open(state.max_upload_size.bytes()).into_bytes().await {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err(Custom(
                Status::PayloadTooLarge,
                Json(ApiResponse {
                    request_id,
                    status: "error".to_string(),
                    message: format!(
                        "Upload size exceeds {} MB limit",
                        state.max_upload_size / 1_048_576
                    ),
                    data: None,
                }),
            ));
        }
    };

    // Process the JSON file
    match process_json_articles(&bytes.value, &request_id, &state.output_base_dir).await {
        Ok((articles_processed, similarity_matrix_shape, proof_size_bytes, output_dir)) => {
            Ok(Json(ApiResponse {
                request_id: request_id.clone(),
                status: "success".to_string(),
                message: "Articles processed successfully".to_string(),
                data: Some(ProcessingResult {
                    articles_processed,
                    similarity_matrix_shape,
                    proof_size_bytes,
                    output_directory: output_dir,
                }),
            }))
        }
        Err(e) => Err(Custom(
            Status::InternalServerError,
            Json(ApiResponse {
                request_id,
                status: "error".to_string(),
                message: format!("Processing failed: {}", e),
                data: None,
            }),
        )),
    }
}

/// Download processing results as a ZIP archive
/// 
/// # Arguments
/// 
/// * `request_id` - The unique request identifier from a previous processing request
#[get("/download/<request_id>")]
pub async fn download_result(
    request_id: &str,
    state: &State<AppState>,
    _api_key: ApiKey,
) -> Result<NamedFile, Custom<Json<ApiResponse>>> {
    let output_dir_name = format!("result_{}", request_id);
    let output_dir = state.output_base_dir.join(&output_dir_name);
    
    if !output_dir.exists() {
        return Err(Custom(
            Status::NotFound,
            Json(ApiResponse {
                request_id: request_id.to_string(),
                status: "error".to_string(),
                message: "Result directory not found. Results may have expired.".to_string(),
                data: None,
            }),
        ));
    }

    // Create a zip file of the results
    let zip_filename = format!("{}.zip", output_dir_name);
    let zip_path = state.output_base_dir.join(&zip_filename);

    if let Err(e) = create_zip_archive(&output_dir, &zip_path) {
        return Err(Custom(
            Status::InternalServerError,
            Json(ApiResponse {
                request_id: request_id.to_string(),
                status: "error".to_string(),
                message: format!("Failed to create zip archive: {}", e),
                data: None,
            }),
        ));
    }

    match NamedFile::open(&zip_path).await {
        Ok(file) => Ok(file),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(ApiResponse {
                request_id: request_id.to_string(),
                status: "error".to_string(),
                message: "Failed to open zip file".to_string(),
                data: None,
            }),
        )),
    }
}

/// Manually trigger cleanup of old result directories
#[post("/cleanup")]
pub async fn manual_cleanup(
    state: &State<AppState>,
    _api_key: ApiKey,
) -> Result<Json<ApiResponse>, Custom<Json<ApiResponse>>> {
    let request_id = Uuid::new_v4().to_string();
    
    match cleanup_old_results(&state.output_base_dir, state.result_ttl_hours).await {
        Ok(()) => Ok(Json(ApiResponse {
            request_id,
            status: "success".to_string(),
            message: format!(
                "Manual cleanup completed. Removed directories older than {} hours.", 
                state.result_ttl_hours
            ),
            data: None,
        })),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            Json(ApiResponse {
                request_id,
                status: "error".to_string(),
                message: format!("Cleanup failed: {}", e),
                data: None,
            }),
        )),
    }
}

/// Error handler for unauthorized requests
#[catch(401)]
pub fn unauthorized() -> Json<ApiResponse> {
    Json(ApiResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "error".to_string(),
        message: "Unauthorized. Valid API key required in X-API-Key header or Authorization: Bearer <key>".to_string(),
        data: None,
    })
}

/// Error handler for not found endpoints
#[catch(404)]
pub fn not_found() -> Json<ApiResponse> {
    Json(ApiResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "error".to_string(),
        message: "Endpoint not found".to_string(),
        data: None,
    })
}

/// Error handler for internal server errors
#[catch(500)]
pub fn internal_error() -> Json<ApiResponse> {
    Json(ApiResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "error".to_string(),
        message: "Internal server error".to_string(),
        data: None,
    })
}

/// Background cleanup task fairing
pub struct CleanupFairing {
    output_base_dir: PathBuf,
    ttl_hours: u64,
}

impl CleanupFairing {
    /// Create a new cleanup fairing
    /// 
    /// # Arguments
    /// 
    /// * `output_base_dir` - Base directory for result cleanup
    /// * `ttl_hours` - Time-to-live for results in hours
    pub fn new(output_base_dir: PathBuf, ttl_hours: u64) -> Self {
        Self { 
            output_base_dir, 
            ttl_hours 
        }
    }
}

#[rocket::async_trait]
impl Fairing for CleanupFairing {
    fn info(&self) -> Info {
        Info {
            name: "Result Cleanup Task",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        let output_dir = self.output_base_dir.clone();
        let ttl = self.ttl_hours;
        
        tokio::spawn(async move {
            start_cleanup_task(output_dir, ttl).await;
        });
        
        println!(
            "Started cleanup task: removing results older than {} hours every {} minutes", 
            ttl, 
            Config::default().cleanup_interval_minutes
        );
        
        Ok(rocket)
    }
}

/// Background task to periodically clean up old results
/// 
/// # Arguments
/// 
/// * `output_base_dir` - Base directory containing result directories
/// * `ttl_hours` - Maximum age for results before cleanup
async fn start_cleanup_task(output_base_dir: PathBuf, ttl_hours: u64) {
    let mut interval = interval(Duration::from_secs(
        Config::default().cleanup_interval_minutes * 60
    ));
    
    loop {
        interval.tick().await;
        
        if let Err(e) = cleanup_old_results(&output_base_dir, ttl_hours).await {
            eprintln!("Cleanup task error: {}", e);
        }
    }
}

/// Process JSON articles through the recommender system
/// 
/// # Arguments
/// 
/// * `json_bytes` - Raw JSON data containing articles
/// * `request_id` - Unique identifier for this processing request
/// * `output_base_dir` - Base directory for saving results
/// 
/// # Returns
/// 
/// A tuple containing:
/// - Number of articles processed
/// - Shape of the similarity matrix (rows, columns)
/// - Size of the proof data in bytes
/// - Output directory path
async fn process_json_articles(
    json_bytes: &[u8],
    request_id: &str,
    output_base_dir: &PathBuf,
) -> Result<(usize, (usize, usize), usize, String)> {
    // Parse JSON content
    let content_json: ContentJson = serde_json::from_slice(json_bytes)
        .context("Failed to parse JSON content")?;

    // Sort articles by date (most recent first) and limit to maximum configured amount
    let mut articles = content_json.posts;
    articles.sort_by(|a, b| {
        // Parse dates for proper sorting - assuming MM/DD/YYYY format
        let parse_date = |date_str: &str| -> Result<chrono::NaiveDate, chrono::ParseError> {
            chrono::NaiveDate::parse_from_str(date_str, "%m/%d/%Y")
        };

        let date_a = parse_date(&a.date)
            .unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let date_b = parse_date(&b.date)
            .unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        date_b.cmp(&date_a) // Most recent first
    });

    let max_articles = Config::default().max_articles_processed;
    if articles.len() > max_articles {
        articles.truncate(max_articles);
    }

    // Create output directory
    let output_dir_name = format!("result_{}", request_id);
    let output_dir = output_base_dir.join(&output_dir_name);
    fs::create_dir_all(&output_dir)
        .context("Failed to create output directory")?;

    // Process articles through recommender system
    let mut recommender = RecommenderSystem::new();
    let (article_ids, proof_data, circuit_settings) = recommender
        .load_and_process_json(&articles)
        .context("Failed to process articles through recommender system")?;

    // Get similarity matrix
    let similarity_matrix = recommender.get_similarity_matrix();

    // Save all outputs
    save_outputs(
        &output_dir,
        &article_ids,
        &similarity_matrix,
        &proof_data,
        &circuit_settings,
        request_id,
    )
    .context("Failed to save processing outputs")?;

    let similarity_matrix_shape = (
        similarity_matrix.len(),
        similarity_matrix.get(0).map_or(0, |row| row.len()),
    );

    Ok((
        article_ids.len(),
        similarity_matrix_shape,
        proof_data.len(),
        output_dir.to_string_lossy().to_string(),
    ))
} 