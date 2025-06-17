#[macro_use]
extern crate rocket;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod recommender;
use recommender::RecommenderSystem;

const MAX_ARTICLES_PROCESSED: usize = 70;

#[derive(Debug, Serialize)]
struct ApiResponse {
    request_id: String,
    status: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<ProcessingResult>,
}

#[derive(Debug, Serialize)]
struct ProcessingResult {
    articles_processed: usize,
    similarity_matrix_shape: (usize, usize),
    proof_size_bytes: usize,
    output_directory: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct ContentJson {
    #[allow(dead_code)]
    timestamp: Option<u64>,
    posts: Vec<recommender::JsonArticle>,
}

struct AppState {
    max_upload_size: usize,
    api_key: String,
    output_base_dir: PathBuf,
}

// API Key request guard
struct ApiKey;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let state = request.guard::<&State<AppState>>().await;

        match state {
            Outcome::Success(app_state) => {
                if let Some(key) = request.headers().get_one("X-API-Key") {
                    if key == app_state.api_key {
                        Outcome::Success(ApiKey)
                    } else {
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                } else if let Some(key) = request.headers().get_one("Authorization") {
                    // Accept Bearer token format
                    if let Some(bearer_key) = key.strip_prefix("Bearer ") {
                        if bearer_key == app_state.api_key {
                            Outcome::Success(ApiKey)
                        } else {
                            Outcome::Error((Status::Unauthorized, ()))
                        }
                    } else {
                        Outcome::Error((Status::Unauthorized, ()))
                    }
                } else {
                    Outcome::Error((Status::Unauthorized, ()))
                }
            }
            _ => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}

#[get("/health")]
fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "rekt-recommender-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[post("/process", data = "<data>")]
async fn process_articles(
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
    match process_json_file(&bytes.value, &request_id, &state.output_base_dir).await {
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

async fn process_json_file(
    json_bytes: &[u8],
    request_id: &str,
    output_base_dir: &Path,
) -> Result<(usize, (usize, usize), usize, String)> {
    // Parse JSON content
    let content_json: ContentJson =
        serde_json::from_slice(json_bytes).context("Failed to parse JSON content")?;

    // Sort articles by date (most recent first) and take the first MAX_ARTICLES_PROCESSED
    let mut articles = content_json.posts;
    articles.sort_by(|a, b| {
        // Parse dates for proper sorting - assuming MM/DD/YYYY format
        let parse_date = |date_str: &str| -> Result<chrono::NaiveDate, chrono::ParseError> {
            chrono::NaiveDate::parse_from_str(date_str, "%m/%d/%Y")
        };

        let date_a =
            parse_date(&a.date).unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
        let date_b =
            parse_date(&b.date).unwrap_or(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());

        date_b.cmp(&date_a) // Most recent first
    });

    if articles.len() > MAX_ARTICLES_PROCESSED {
        articles.truncate(MAX_ARTICLES_PROCESSED);
    }

    // Create output directory
    let output_dir_name = format!("result_{}", request_id);
    let output_dir = output_base_dir.join(&output_dir_name);
    fs::create_dir_all(&output_dir).context("Failed to create output directory")?;

    // Process articles
    let mut recommender = RecommenderSystem::new();
    let (article_ids, proof_data, circuit_settings) = recommender
        .load_and_process_json(&articles)
        .context("Failed to process articles")?;

    // Get similarity matrix
    let similarity_matrix = recommender.get_similarity_matrix();

    // Save outputs
    save_outputs(
        &output_dir,
        &article_ids,
        &similarity_matrix,
        &proof_data,
        &circuit_settings,
        request_id,
    )
    .context("Failed to save outputs")?;

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

fn save_outputs(
    output_dir: &Path,
    article_ids: &[String],
    similarity_matrix: &Vec<Vec<f32>>,
    proof_data: &[u8],
    circuit_settings: &[u8],
    request_id: &str,
) -> Result<()> {
    // Save article IDs
    let ids_path = output_dir.join("article_ids.json");
    let ids_json = serde_json::to_string_pretty(&article_ids)?;
    fs::write(&ids_path, ids_json)?;

    // Save similarity matrix
    let matrix_path = output_dir.join("similarity_matrix.json");
    let matrix_json = serde_json::to_string_pretty(&similarity_matrix)?;
    fs::write(&matrix_path, matrix_json)?;

    // Save proof as binary
    let proof_path = output_dir.join("proof.bin");
    fs::write(&proof_path, proof_data)?;

    // Save circuit settings as circuit_settings.bin
    let settings_path = output_dir.join("circuit_settings.bin");
    fs::write(&settings_path, circuit_settings)?;

    // Save metadata
    let metadata = serde_json::json!({
        "request_id": request_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "articles_processed": article_ids.len(),
        "matrix_shape": [similarity_matrix.len(), similarity_matrix.get(0).map_or(0, |row| row.len())],
        "proof_size_bytes": proof_data.len(),
        "settings_size_bytes": circuit_settings.len(),
    });
    let metadata_path = output_dir.join("metadata.json");
    fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;

    Ok(())
}

#[catch(401)]
fn unauthorized() -> Json<ApiResponse> {
    Json(ApiResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "error".to_string(),
        message: "Unauthorized. Valid API key required in X-API-Key header or Authorization: Bearer <key>".to_string(),
        data: None,
    })
}

#[catch(404)]
fn not_found() -> Json<ApiResponse> {
    Json(ApiResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "error".to_string(),
        message: "Endpoint not found".to_string(),
        data: None,
    })
}

#[catch(500)]
fn internal_error() -> Json<ApiResponse> {
    Json(ApiResponse {
        request_id: Uuid::new_v4().to_string(),
        status: "error".to_string(),
        message: "Internal server error".to_string(),
        data: None,
    })
}

#[launch]
fn rocket() -> _ {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let max_upload_size = std::env::var("MAX_UPLOAD_SIZE_MB")
        .unwrap_or_else(|_| "100".to_string())
        .parse::<usize>()
        .expect("MAX_UPLOAD_SIZE_MB must be a valid usize")
        * 1_048_576; // Convert MB to bytes

    let api_key = std::env::var("API_KEY").expect("API_KEY environment variable must be set");

    if api_key.len() < 16 {
        panic!("API_KEY must be at least 16 characters long for security");
    }

    let output_base_dir = std::env::var("OUTPUT_DIR")
        .unwrap_or_else(|_| "./outputs".to_string())
        .into();

    // Create output base directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(&output_base_dir) {
        panic!("Failed to create output base directory: {}", e);
    }

    rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .manage(AppState {
            max_upload_size,
            api_key,
            output_base_dir,
        })
        .mount("/", routes![health, process_articles])
        .register("/", catchers![unauthorized, not_found, internal_error])
}
