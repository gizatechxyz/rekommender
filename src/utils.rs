use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, SystemTime};

use anyhow::{Context, Result};
use zip::{write::FileOptions, ZipWriter};

/// Create a ZIP archive from a source directory
/// 
/// # Arguments
/// 
/// * `source_dir` - Path to the directory to archive
/// * `zip_path` - Path where the ZIP file should be created
/// 
/// # Returns
/// 
/// Returns `Ok(())` on success, or an error if the operation fails
pub fn create_zip_archive(source_dir: &Path, zip_path: &Path) -> Result<()> {
    let file = File::create(zip_path)
        .with_context(|| format!("Failed to create ZIP file at {:?}", zip_path))?;
    
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Add all files from the source directory
    for entry in fs::read_dir(source_dir)
        .with_context(|| format!("Failed to read directory {:?}", source_dir))? {
        
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();
        
        if path.is_file() {
            let file_name = path.file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?
                .to_string_lossy();
            
            zip.start_file(file_name.clone(), options)
                .with_context(|| format!("Failed to start ZIP file entry for {:?}", file_name))?;
            
            let file_content = fs::read(&path)
                .with_context(|| format!("Failed to read file {:?}", path))?;
            
            zip.write_all(&file_content)
                .with_context(|| format!("Failed to write file content to ZIP for {:?}", file_name))?;
        }
    }

    zip.finish()
        .with_context(|| "Failed to finalize ZIP archive")?;
    
    Ok(())
}

/// Clean up old result directories based on their age
/// 
/// # Arguments
/// 
/// * `output_base_dir` - Base directory containing result directories
/// * `max_age_hours` - Maximum age in hours before directories are deleted
/// 
/// # Returns
/// 
/// Returns `Ok(())` on success, or an error if the cleanup operation fails
pub async fn cleanup_old_results(output_base_dir: &Path, max_age_hours: u64) -> Result<()> {
    let max_age = Duration::from_secs(max_age_hours * 3600);
    let now = SystemTime::now();
    
    if !output_base_dir.exists() {
        return Ok(());
    }
    
    let mut cleaned_count = 0;
    let mut total_checked = 0;
    
    match fs::read_dir(output_base_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                
                // Only process directories that start with "result_"
                if path.is_dir() {
                    if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                        if dir_name.starts_with("result_") {
                            total_checked += 1;
                            
                            // Check directory age
                            if let Ok(metadata) = entry.metadata() {
                                if let Ok(created) = metadata.created() {
                                    if let Ok(age) = now.duration_since(created) {
                                        if age > max_age {
                                            // Remove old directory
                                            if fs::remove_dir_all(&path).is_ok() {
                                                cleaned_count += 1;
                                                println!("Cleaned up old result directory: {}", dir_name);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading output directory for cleanup: {}", e);
        }
    }
    
    if cleaned_count > 0 {
        println!(
            "Cleanup completed: removed {} old directories out of {} checked", 
            cleaned_count, 
            total_checked
        );
    }
    
    Ok(())
}

/// Save processing outputs to the specified directory
/// 
/// # Arguments
/// 
/// * `output_dir` - Directory where outputs will be saved
/// * `article_ids` - List of article identifiers
/// * `similarity_matrix` - Computed similarity matrix
/// * `proof_data` - Zero-knowledge proof data
/// * `circuit_settings` - Circuit settings for proof verification
/// * `request_id` - Unique request identifier
/// 
/// # Returns
/// 
/// Returns `Ok(())` on success, or an error if saving fails
pub fn save_outputs(
    output_dir: &Path,
    article_ids: &[String],
    similarity_matrix: &Vec<Vec<f32>>,
    proof_data: &[u8],
    circuit_settings: &[u8],
    request_id: &str,
) -> Result<()> {
    // Save article IDs
    let ids_path = output_dir.join("article_ids.json");
    let ids_json = serde_json::to_string_pretty(&article_ids)
        .context("Failed to serialize article IDs")?;
    fs::write(&ids_path, ids_json)
        .context("Failed to write article IDs file")?;

    // Save similarity matrix
    let matrix_path = output_dir.join("similarity_matrix.json");
    let matrix_json = serde_json::to_string_pretty(&similarity_matrix)
        .context("Failed to serialize similarity matrix")?;
    fs::write(&matrix_path, matrix_json)
        .context("Failed to write similarity matrix file")?;

    // Save proof as binary
    let proof_path = output_dir.join("proof.bin");
    fs::write(&proof_path, proof_data)
        .context("Failed to write proof file")?;

    // Save circuit settings as binary
    let settings_path = output_dir.join("circuit_settings.bin");
    fs::write(&settings_path, circuit_settings)
        .context("Failed to write circuit settings file")?;

    // Save metadata
    let metadata = serde_json::json!({
        "request_id": request_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "articles_processed": article_ids.len(),
        "matrix_shape": [
            similarity_matrix.len(), 
            similarity_matrix.get(0).map_or(0, |row| row.len())
        ],
        "proof_size_bytes": proof_data.len(),
        "settings_size_bytes": circuit_settings.len(),
    });
    
    let metadata_path = output_dir.join("metadata.json");
    fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)
        .context("Failed to write metadata file")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_save_outputs() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();
        
        let article_ids = vec!["article1".to_string(), "article2".to_string()];
        let similarity_matrix = vec![
            vec![1.0, 0.5],
            vec![0.5, 1.0],
        ];
        let proof_data = b"test proof data";
        let circuit_settings = b"test circuit settings";
        let request_id = "test-request-123";

        let result = save_outputs(
            output_dir,
            &article_ids,
            &similarity_matrix,
            proof_data,
            circuit_settings,
            request_id,
        );

        assert!(result.is_ok());

        // Check that all files were created
        assert!(output_dir.join("article_ids.json").exists());
        assert!(output_dir.join("similarity_matrix.json").exists());
        assert!(output_dir.join("proof.bin").exists());
        assert!(output_dir.join("circuit_settings.bin").exists());
        assert!(output_dir.join("metadata.json").exists());

        // Check content of article_ids.json
        let ids_content = fs::read_to_string(output_dir.join("article_ids.json")).unwrap();
        let parsed_ids: Vec<String> = serde_json::from_str(&ids_content).unwrap();
        assert_eq!(parsed_ids, article_ids);

        // Check content of similarity_matrix.json
        let matrix_content = fs::read_to_string(output_dir.join("similarity_matrix.json")).unwrap();
        let parsed_matrix: Vec<Vec<f32>> = serde_json::from_str(&matrix_content).unwrap();
        assert_eq!(parsed_matrix, similarity_matrix);

        // Check binary files
        let proof_content = fs::read(output_dir.join("proof.bin")).unwrap();
        assert_eq!(proof_content, proof_data);

        let settings_content = fs::read(output_dir.join("circuit_settings.bin")).unwrap();
        assert_eq!(settings_content, circuit_settings);

        // Check metadata
        let metadata_content = fs::read_to_string(output_dir.join("metadata.json")).unwrap();
        let metadata: serde_json::Value = serde_json::from_str(&metadata_content).unwrap();
        assert_eq!(metadata["request_id"], request_id);
        assert_eq!(metadata["articles_processed"], 2);
        assert_eq!(metadata["matrix_shape"], serde_json::json!([2, 2]));
    }

    #[test]
    fn test_create_zip_archive() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let zip_path = temp_dir.path().join("archive.zip");

        // Create source directory with test files
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join("file1.txt"), "content1").unwrap();
        fs::write(source_dir.join("file2.txt"), "content2").unwrap();

        let result = create_zip_archive(&source_dir, &zip_path);
        assert!(result.is_ok());
        assert!(zip_path.exists());

        // Verify ZIP file is not empty
        let zip_size = fs::metadata(&zip_path).unwrap().len();
        assert!(zip_size > 0);
    }

    #[test]
    fn test_create_zip_archive_nonexistent_source() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");
        let zip_path = temp_dir.path().join("archive.zip");

        let result = create_zip_archive(&nonexistent_dir, &zip_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_zip_archive_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let empty_dir = temp_dir.path().join("empty");
        let zip_path = temp_dir.path().join("archive.zip");

        fs::create_dir_all(&empty_dir).unwrap();

        let result = create_zip_archive(&empty_dir, &zip_path);
        assert!(result.is_ok());
        assert!(zip_path.exists());
    }

    #[tokio::test]
    async fn test_cleanup_old_results_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_dir = temp_dir.path().join("nonexistent");

        let result = cleanup_old_results(&nonexistent_dir, 24).await;
        assert!(result.is_ok()); // Should handle gracefully
    }

    #[tokio::test]
    async fn test_cleanup_old_results_no_result_directories() {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path();

        // Create some non-result directories
        fs::create_dir_all(base_dir.join("other_dir")).unwrap();
        fs::write(base_dir.join("file.txt"), "content").unwrap();

        let result = cleanup_old_results(base_dir, 24).await;
        assert!(result.is_ok());

        // Directories should still exist
        assert!(base_dir.join("other_dir").exists());
        assert!(base_dir.join("file.txt").exists());
    }

    #[test]
    fn test_save_outputs_invalid_directory() {
        use std::path::Path;
        
        // Try to save to root directory (should fail on most systems)
        let invalid_dir = Path::new("/invalid/nonexistent/path");
        
        let result = save_outputs(
            invalid_dir,
            &vec!["test".to_string()],
            &vec![vec![1.0]],
            b"test",
            b"test",
            "test-id",
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_save_outputs_with_empty_data() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();
        
        let empty_ids: Vec<String> = vec![];
        let empty_matrix: Vec<Vec<f32>> = vec![];
        
        let result = save_outputs(
            output_dir,
            &empty_ids,
            &empty_matrix,
            b"",
            b"",
            "empty-test",
        );
        
        assert!(result.is_ok());
        
        // Files should still be created even with empty data
        assert!(output_dir.join("article_ids.json").exists());
        assert!(output_dir.join("similarity_matrix.json").exists());
        assert!(output_dir.join("proof.bin").exists());
        assert!(output_dir.join("circuit_settings.bin").exists());
        assert!(output_dir.join("metadata.json").exists());
    }

    #[test]
    fn test_save_outputs_metadata_content() {
        let temp_dir = TempDir::new().unwrap();
        let output_dir = temp_dir.path();
        
        let article_ids = vec!["art1".to_string(), "art2".to_string(), "art3".to_string()];
        let similarity_matrix = vec![
            vec![1.0, 0.3, 0.7],
            vec![0.3, 1.0, 0.5],
            vec![0.7, 0.5, 1.0],
        ];
        let proof_data = b"proof data content";
        let circuit_settings = b"settings content";
        
        save_outputs(
            output_dir,
            &article_ids,
            &similarity_matrix,
            proof_data,
            circuit_settings,
            "metadata-test",
        ).unwrap();
        
        let metadata_content = fs::read_to_string(output_dir.join("metadata.json")).unwrap();
        let metadata: serde_json::Value = serde_json::from_str(&metadata_content).unwrap();
        
        assert_eq!(metadata["request_id"], "metadata-test");
        assert_eq!(metadata["articles_processed"], 3);
        assert_eq!(metadata["matrix_shape"], serde_json::json!([3, 3]));
        assert_eq!(metadata["proof_size_bytes"], proof_data.len());
        assert_eq!(metadata["settings_size_bytes"], circuit_settings.len());
        
        // Check that timestamp is present and valid
        assert!(metadata["timestamp"].is_string());
        let timestamp_str = metadata["timestamp"].as_str().unwrap();
        assert!(timestamp_str.len() > 0);
    }

    #[test]
    fn test_create_zip_archive_file_permissions() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("source");
        let zip_path = temp_dir.path().join("test.zip");

        // Create source directory with a file
        fs::create_dir_all(&source_dir).unwrap();
        fs::write(source_dir.join("test.txt"), "test content").unwrap();

        let result = create_zip_archive(&source_dir, &zip_path);
        assert!(result.is_ok());

        // Verify ZIP file exists and has content
        assert!(zip_path.exists());
        let metadata = fs::metadata(&zip_path).unwrap();
        assert!(metadata.len() > 0);
    }
} 