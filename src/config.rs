use std::path::PathBuf;

/// Configuration constants for the recommender system
pub struct Config {
    /// Cleanup interval in minutes
    pub cleanup_interval_minutes: u64,
    /// Maximum number of articles to process
    pub max_articles_processed: usize,
    /// Default result TTL in hours
    pub default_result_ttl_hours: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cleanup_interval_minutes: 60,
            max_articles_processed: 70,
            default_result_ttl_hours: 24,
        }
    }
}

/// Text processing configuration
pub struct TextProcessingConfig {
    /// Boost factor for tags in TF-IDF calculation
    pub tag_boost: usize,
    /// Boost factor for auditor information
    pub auditor_boost: usize,
    /// Maximum percentage of documents a term can appear in
    pub max_doc_percentage: f32,
    /// Minimum document threshold for unigrams
    pub min_doc_threshold_unigram: usize,
    /// Minimum document threshold for n-grams
    pub min_doc_threshold_ngram: usize,
}

impl Default for TextProcessingConfig {
    fn default() -> Self {
        Self {
            tag_boost: 5,
            auditor_boost: 3,
            max_doc_percentage: 0.80,
            min_doc_threshold_unigram: 5,
            min_doc_threshold_ngram: 2,
        }
    }
}

/// Application state configuration
pub struct AppState {
    pub max_upload_size: usize,
    pub api_key: String,
    pub output_base_dir: PathBuf,
    pub result_ttl_hours: u64,
}

/// Application configuration loaded from environment variables
pub struct AppConfig {
    pub port: u16,
    pub max_upload_size: usize,
    pub api_key: String,
    pub output_base_dir: PathBuf,
    pub result_ttl_hours: u64,
}

impl AppConfig {
    /// Load configuration from environment variables
    /// 
    /// # Returns
    /// 
    /// Returns `AppConfig` with values loaded from environment variables or defaults
    /// 
    /// # Panics
    /// 
    /// Panics if required environment variables are missing or invalid
    pub fn from_env() -> Self {
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

        let output_base_dir: PathBuf = std::env::var("OUTPUT_DIR")
            .unwrap_or_else(|_| "./outputs".to_string())
            .into();

        let result_ttl_hours = std::env::var("RESULT_TTL_HOURS")
            .unwrap_or_else(|_| Config::default().default_result_ttl_hours.to_string())
            .parse::<u64>()
            .expect("RESULT_TTL_HOURS must be a valid u64");

        Self {
            port,
            max_upload_size,
            api_key,
            output_base_dir,
            result_ttl_hours,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use std::panic;

    // Use a mutex to serialize tests that modify environment variables
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    fn with_clean_env<F>(f: F) 
    where 
        F: FnOnce() + panic::UnwindSafe,
    {
        let _guard = ENV_MUTEX.lock().unwrap_or_else(|poisoned| {
            // If mutex is poisoned, clear the poison and continue
            poisoned.into_inner()
        });
        
        // Save current environment
        let orig_port = env::var("PORT");
        let orig_size = env::var("MAX_UPLOAD_SIZE_MB");
        let orig_dir = env::var("OUTPUT_DIR");
        let orig_ttl = env::var("RESULT_TTL_HOURS");
        let orig_key = env::var("API_KEY");
        
        // Clear all relevant environment variables
        env::remove_var("PORT");
        env::remove_var("MAX_UPLOAD_SIZE_MB");
        env::remove_var("OUTPUT_DIR");
        env::remove_var("RESULT_TTL_HOURS");
        env::remove_var("API_KEY");
        
        // Run the test and catch any panics to ensure cleanup
        let result = panic::catch_unwind(f);
        
        // Restore original environment regardless of whether test panicked
        match orig_port {
            Ok(val) => env::set_var("PORT", val),
            Err(_) => env::remove_var("PORT"),
        }
        match orig_size {
            Ok(val) => env::set_var("MAX_UPLOAD_SIZE_MB", val),
            Err(_) => env::remove_var("MAX_UPLOAD_SIZE_MB"),
        }
        match orig_dir {
            Ok(val) => env::set_var("OUTPUT_DIR", val),
            Err(_) => env::remove_var("OUTPUT_DIR"),
        }
        match orig_ttl {
            Ok(val) => env::set_var("RESULT_TTL_HOURS", val),
            Err(_) => env::remove_var("RESULT_TTL_HOURS"),
        }
        match orig_key {
            Ok(val) => env::set_var("API_KEY", val),
            Err(_) => env::remove_var("API_KEY"),
        }
        
        // Re-panic if the test panicked
        if let Err(panic) = result {
            panic::resume_unwind(panic);
        }
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.cleanup_interval_minutes, 60);
        assert_eq!(config.max_articles_processed, 70);
        assert_eq!(config.default_result_ttl_hours, 24);
    }

    #[test]
    fn test_text_processing_config_default() {
        let config = TextProcessingConfig::default();
        assert_eq!(config.tag_boost, 5);
        assert_eq!(config.auditor_boost, 3);
        assert_eq!(config.max_doc_percentage, 0.80);
        assert_eq!(config.min_doc_threshold_unigram, 5);
        assert_eq!(config.min_doc_threshold_ngram, 2);
    }

    #[test]
    fn test_app_config_from_env_with_defaults() {
        with_clean_env(|| {
            // Set required API key
            env::set_var("API_KEY", "test-api-key-1234567890");

            let config = AppConfig::from_env();
            assert_eq!(config.port, 8080);
            assert_eq!(config.max_upload_size, 100 * 1_048_576); // 100MB in bytes
            assert_eq!(config.api_key, "test-api-key-1234567890");
            assert_eq!(config.output_base_dir, std::path::PathBuf::from("./outputs"));
            assert_eq!(config.result_ttl_hours, 24);
        });
    }

    #[test]
    fn test_app_config_from_env_with_custom_values() {
        with_clean_env(|| {
            // Set custom environment variables
            env::set_var("PORT", "3000");
            env::set_var("MAX_UPLOAD_SIZE_MB", "200");
            env::set_var("API_KEY", "custom-api-key-abcdefghijklmnop");
            env::set_var("OUTPUT_DIR", "/custom/output");
            env::set_var("RESULT_TTL_HOURS", "48");

            let config = AppConfig::from_env();
            assert_eq!(config.port, 3000);
            assert_eq!(config.max_upload_size, 200 * 1_048_576); // 200MB in bytes
            assert_eq!(config.api_key, "custom-api-key-abcdefghijklmnop");
            assert_eq!(config.output_base_dir, std::path::PathBuf::from("/custom/output"));
            assert_eq!(config.result_ttl_hours, 48);
        });
    }

    #[test]
    #[should_panic(expected = "API_KEY environment variable must be set")]
    fn test_app_config_missing_api_key() {
        with_clean_env(|| {
            AppConfig::from_env();
        });
    }

    #[test]
    #[should_panic(expected = "API_KEY must be at least 16 characters long")]
    fn test_app_config_short_api_key() {
        with_clean_env(|| {
            env::set_var("API_KEY", "short");
            AppConfig::from_env();
        });
    }

    #[test]
    #[should_panic(expected = "PORT must be a valid u16")]
    fn test_app_config_invalid_port() {
        with_clean_env(|| {
            env::set_var("PORT", "invalid");
            env::set_var("API_KEY", "valid-api-key-1234567890");
            AppConfig::from_env();
        });
    }

    #[test]
    #[should_panic(expected = "MAX_UPLOAD_SIZE_MB must be a valid usize")]
    fn test_app_config_invalid_upload_size() {
        with_clean_env(|| {
            env::set_var("MAX_UPLOAD_SIZE_MB", "invalid");
            env::set_var("API_KEY", "valid-api-key-1234567890");
            AppConfig::from_env();
        });
    }

    #[test]
    #[should_panic(expected = "RESULT_TTL_HOURS must be a valid u64")]
    fn test_app_config_invalid_ttl() {
        with_clean_env(|| {
            env::set_var("RESULT_TTL_HOURS", "invalid");
            env::set_var("API_KEY", "valid-api-key-1234567890");
            AppConfig::from_env();
        });
    }

    #[test]
    fn test_app_config_edge_case_values() {
        with_clean_env(|| {
            // Test with minimum valid API key length
            env::set_var("API_KEY", "exactly16chars12");
            env::set_var("PORT", "1");
            env::set_var("MAX_UPLOAD_SIZE_MB", "1");
            env::set_var("RESULT_TTL_HOURS", "1");

            let config = AppConfig::from_env();
            assert_eq!(config.api_key, "exactly16chars12");
            assert_eq!(config.port, 1);
            assert_eq!(config.max_upload_size, 1_048_576); // 1MB
            assert_eq!(config.result_ttl_hours, 1);
        });
    }

    #[test]
    fn test_app_config_very_large_values() {
        with_clean_env(|| {
            env::set_var("API_KEY", "very-long-api-key-that-is-definitely-more-than-16-characters");
            env::set_var("PORT", "65535"); // Max u16 value
            env::set_var("MAX_UPLOAD_SIZE_MB", "1000");
            env::set_var("RESULT_TTL_HOURS", "8760"); // 1 year in hours

            let config = AppConfig::from_env();
            assert_eq!(config.port, 65535);
            assert_eq!(config.max_upload_size, 1000 * 1_048_576);
            assert_eq!(config.result_ttl_hours, 8760);
        });
    }

    #[test]
    fn test_text_processing_config_custom_values() {
        let config = TextProcessingConfig {
            tag_boost: 10,
            auditor_boost: 7,
            max_doc_percentage: 0.5,
            min_doc_threshold_unigram: 3,
            min_doc_threshold_ngram: 1,
        };

        assert_eq!(config.tag_boost, 10);
        assert_eq!(config.auditor_boost, 7);
        assert_eq!(config.max_doc_percentage, 0.5);
        assert_eq!(config.min_doc_threshold_unigram, 3);
        assert_eq!(config.min_doc_threshold_ngram, 1);
    }
} 