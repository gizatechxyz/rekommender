#[macro_use]
extern crate rocket;

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use anyhow::{Context, Result};
use rocket::data::{Data, ToByteUnit};
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use serde::Serialize;
use tempfile::TempDir;
use uuid::Uuid;
use zip::write::FileOptions;

mod recommender;
use recommender::RecommenderSystem;

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
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
}

struct AppState {
    max_upload_size: usize,
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
) -> Result<(Status, Vec<u8>), Custom<Json<ApiResponse>>> {
    let request_id = Uuid::new_v4().to_string();

    // Verify content type
    if !content_type.is_zip() {
        return Err(Custom(
            Status::BadRequest,
            Json(ApiResponse {
                request_id,
                status: "error".to_string(),
                message: "Content-Type must be application/zip".to_string(),
                data: None,
            }),
        ));
    }

    // Read the uploaded data
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

    // Process the zip file
    match process_zip_file(&bytes.value, &request_id).await {
        Ok(result_bytes) => Ok((Status::Ok, result_bytes)),
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

async fn process_zip_file(zip_bytes: &[u8], request_id: &str) -> Result<Vec<u8>> {
    // Create temporary directories
    let temp_dir = TempDir::new().context("Failed to create temp directory")?;
    let input_dir = temp_dir.path().join("input");
    let output_dir = temp_dir.path().join("output");

    fs::create_dir(&input_dir).context("Failed to create input directory")?;
    fs::create_dir(&output_dir).context("Failed to create output directory")?;

    // Extract input zip
    extract_zip(zip_bytes, &input_dir).context("Failed to extract input zip")?;

    // Process articles
    let mut recommender = RecommenderSystem::new();
    let (article_ids, proof_data) = recommender
        .load_and_process(&input_dir)
        .context("Failed to process articles")?;

    // Get similarity matrix
    let similarity_matrix = recommender.get_similarity_matrix();

    // Save outputs
    save_outputs(
        &output_dir,
        &article_ids,
        &similarity_matrix,
        &proof_data,
        request_id,
    )
    .context("Failed to save outputs")?;

    // Create output zip
    create_output_zip(&output_dir).context("Failed to create output zip")
}

fn extract_zip(zip_bytes: &[u8], output_dir: &Path) -> Result<()> {
    use std::io::Cursor;
    use zip::ZipArchive;

    let cursor = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = output_dir.join(file.mangled_name());

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn save_outputs(
    output_dir: &Path,
    article_ids: &[String],
    similarity_matrix: &Vec<Vec<f32>>,
    proof_data: &[u8],
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

    // Save metadata
    let metadata = serde_json::json!({
        "request_id": request_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "articles_processed": article_ids.len(),
        "matrix_shape": [similarity_matrix.len(), similarity_matrix.get(0).map_or(0, |row| row.len())],
        "proof_size_bytes": proof_data.len(),
    });
    let metadata_path = output_dir.join("metadata.json");
    fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;

    Ok(())
}

fn create_output_zip(output_dir: &Path) -> Result<Vec<u8>> {
    use std::io::Cursor;
    use zip::ZipWriter;

    let mut buffer = Vec::new();
    {
        let cursor = Cursor::new(&mut buffer);
        let mut zip = ZipWriter::new(cursor);
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        // Add all files from output directory
        for entry in fs::read_dir(output_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy();
                zip.start_file(file_name, options)?;
                let contents = fs::read(&path)?;
                zip.write_all(&contents)?;
            }
        }

        zip.finish()?;
    }

    Ok(buffer)
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

    rocket::build()
        .configure(rocket::Config {
            port,
            address: std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            ..Default::default()
        })
        .manage(AppState { max_upload_size })
        .mount("/", routes![health, process_articles])
        .register("/", catchers![not_found, internal_error])
}
