use serde::{Deserialize, Serialize};

/// JSON structure for article data input
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonArticle {
    /// Publication date of the article (MM/DD/YYYY format)
    pub date: String,
    /// Whether the article is featured (optional)
    #[allow(dead_code)]
    pub featured: Option<bool>,
    /// Article title
    pub title: String,
    /// Rekt-specific details (optional)
    pub rekt: Option<RektDetails>,
    /// Article tags for categorization (optional)
    pub tags: Option<Vec<String>>,
    /// Article excerpt or summary (optional)
    pub excerpt: Option<String>,
    /// Banner image (optional)
    #[allow(dead_code)]
    pub banner: Option<String>,
    /// URL slug for the article
    pub slug: String,
}

/// Details specific to Rekt articles
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RektDetails {
    /// Amount lost in the incident (optional)
    #[allow(dead_code)]
    pub amount: Option<u64>,
    /// Audit information (optional)
    pub audit: Option<String>,
    /// Date of the incident (optional)
    #[allow(dead_code)]
    pub date: Option<String>,
}

/// JSON structure for content input containing posts
#[derive(Debug, Serialize, Deserialize)]
pub struct ContentJson {
    /// Timestamp when the content was created (optional)
    #[allow(dead_code)]
    pub timestamp: Option<u64>,
    /// List of articles/posts
    pub posts: Vec<JsonArticle>,
}

/// API response structure
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    /// Unique identifier for the request
    pub request_id: String,
    /// Status of the operation ("success" or "error")
    pub status: String,
    /// Human-readable message describing the result
    pub message: String,
    /// Processing result data (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProcessingResult>,
}

/// Results from processing articles
#[derive(Debug, Serialize)]
pub struct ProcessingResult {
    /// Number of articles that were processed
    pub articles_processed: usize,
    /// Shape of the generated similarity matrix (rows, columns)
    pub similarity_matrix_shape: (usize, usize),
    /// Size of the generated proof in bytes
    pub proof_size_bytes: usize,
    /// Path to the output directory
    pub output_directory: String,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Service name
    pub service: String,
    /// Service version
    pub version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    fn create_test_article() -> JsonArticle {
        JsonArticle {
            date: "01/01/2024".to_string(),
            featured: Some(true),
            title: "Test Article".to_string(),
            rekt: Some(RektDetails {
                amount: Some(1000000),
                audit: Some("Trail of Bits".to_string()),
                date: Some("12/31/2023".to_string()),
            }),
            tags: Some(vec!["DeFi".to_string(), "Security".to_string()]),
            excerpt: Some("Test excerpt".to_string()),
            banner: Some("banner.jpg".to_string()),
            slug: "test-article".to_string(),
        }
    }

    #[test]
    fn test_json_article_serialization() {
        let article = create_test_article();
        let json = serde_json::to_string(&article).unwrap();
        
        assert!(json.contains("\"date\":\"01/01/2024\""));
        assert!(json.contains("\"title\":\"Test Article\""));
        assert!(json.contains("\"slug\":\"test-article\""));
    }

    #[test]
    fn test_json_article_deserialization() {
        let json = r#"{
            "date": "01/01/2024",
            "featured": true,
            "title": "Test Article",
            "rekt": {
                "amount": 1000000,
                "audit": "Trail of Bits",
                "date": "12/31/2023"
            },
            "tags": ["DeFi", "Security"],
            "excerpt": "Test excerpt",
            "banner": "banner.jpg",
            "slug": "test-article"
        }"#;

        let article: JsonArticle = serde_json::from_str(json).unwrap();
        assert_eq!(article.date, "01/01/2024");
        assert_eq!(article.title, "Test Article");
        assert_eq!(article.slug, "test-article");
        assert_eq!(article.featured, Some(true));
        assert!(article.rekt.is_some());
        assert!(article.tags.is_some());
        assert_eq!(article.tags.unwrap(), vec!["DeFi", "Security"]);
    }

    #[test]
    fn test_json_article_minimal_fields() {
        let json = r#"{
            "date": "01/01/2024",
            "title": "Minimal Article",
            "slug": "minimal-article"
        }"#;

        let article: JsonArticle = serde_json::from_str(json).unwrap();
        assert_eq!(article.date, "01/01/2024");
        assert_eq!(article.title, "Minimal Article");
        assert_eq!(article.slug, "minimal-article");
        assert_eq!(article.featured, None);
        assert_eq!(article.rekt, None);
        assert_eq!(article.tags, None);
        assert_eq!(article.excerpt, None);
        assert_eq!(article.banner, None);
    }

    #[test]
    fn test_rekt_details_serialization() {
        let rekt = RektDetails {
            amount: Some(5000000),
            audit: Some("Consensys".to_string()),
            date: Some("01/01/2024".to_string()),
        };

        let json = serde_json::to_string(&rekt).unwrap();
        assert!(json.contains("\"amount\":5000000"));
        assert!(json.contains("\"audit\":\"Consensys\""));
    }

    #[test]
    fn test_rekt_details_deserialization() {
        let json = r#"{
            "amount": 5000000,
            "audit": "Consensys",
            "date": "01/01/2024"
        }"#;

        let rekt: RektDetails = serde_json::from_str(json).unwrap();
        assert_eq!(rekt.amount, Some(5000000));
        assert_eq!(rekt.audit, Some("Consensys".to_string()));
        assert_eq!(rekt.date, Some("01/01/2024".to_string()));
    }

    #[test]
    fn test_rekt_details_optional_fields() {
        let json = r#"{}"#;
        let rekt: RektDetails = serde_json::from_str(json).unwrap();
        assert_eq!(rekt.amount, None);
        assert_eq!(rekt.audit, None);
        assert_eq!(rekt.date, None);
    }

    #[test]
    fn test_content_json_serialization() {
        let content = ContentJson {
            timestamp: Some(1640995200),
            posts: vec![create_test_article()],
        };

        let json = serde_json::to_string(&content).unwrap();
        assert!(json.contains("\"timestamp\":1640995200"));
        assert!(json.contains("\"posts\":["));
    }

    #[test]
    fn test_content_json_deserialization() {
        let json = r#"{
            "timestamp": 1640995200,
            "posts": [
                {
                    "date": "01/01/2024",
                    "title": "Test Article",
                    "slug": "test-article"
                }
            ]
        }"#;

        let content: ContentJson = serde_json::from_str(json).unwrap();
        assert_eq!(content.timestamp, Some(1640995200));
        assert_eq!(content.posts.len(), 1);
        assert_eq!(content.posts[0].title, "Test Article");
    }

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse {
            request_id: "test-123".to_string(),
            status: "success".to_string(),
            message: "Operation completed".to_string(),
            data: Some(ProcessingResult {
                articles_processed: 5,
                similarity_matrix_shape: (5, 5),
                proof_size_bytes: 1024,
                output_directory: "/path/to/output".to_string(),
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"success\""));
        assert!(json.contains("\"request_id\":\"test-123\""));
        assert!(json.contains("\"data\":{"));
    }

    #[test]
    fn test_api_response_error() {
        let response = ApiResponse {
            request_id: "error-123".to_string(),
            status: "error".to_string(),
            message: "Something went wrong".to_string(),
            data: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"error\""));
        assert!(json.contains("\"message\":\"Something went wrong\""));
        // data field should be omitted when None due to skip_serializing_if
        assert!(!json.contains("\"data\""));
    }

    #[test]
    fn test_processing_result_serialization() {
        let result = ProcessingResult {
            articles_processed: 10,
            similarity_matrix_shape: (10, 10),
            proof_size_bytes: 2048,
            output_directory: "/tmp/results".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"articles_processed\":10"));
        assert!(json.contains("\"similarity_matrix_shape\":[10,10]"));
        assert!(json.contains("\"proof_size_bytes\":2048"));
        assert!(json.contains("\"output_directory\":\"/tmp/results\""));
    }

    #[test]
    fn test_health_response_serialization() {
        let response = HealthResponse {
            status: "healthy".to_string(),
            service: "rekt-recommender-api".to_string(),
            version: "0.1.3".to_string(),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"status\":\"healthy\""));
        assert!(json.contains("\"service\":\"rekt-recommender-api\""));
        assert!(json.contains("\"version\":\"0.1.3\""));
    }

    #[test]
    fn test_json_article_with_empty_tags() {
        let json = r#"{
            "date": "01/01/2024",
            "title": "Article with Empty Tags",
            "slug": "article-empty-tags",
            "tags": []
        }"#;

        let article: JsonArticle = serde_json::from_str(json).unwrap();
        assert!(article.tags.is_some());
        assert_eq!(article.tags.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_complex_content_json() {
        let json = r#"{
            "timestamp": 1640995200,
            "posts": [
                {
                    "date": "01/01/2024",
                    "featured": true,
                    "title": "First Article",
                    "rekt": {
                        "amount": 1000000,
                        "audit": "Trail of Bits"
                    },
                    "tags": ["DeFi", "Security"],
                    "excerpt": "First excerpt",
                    "slug": "first-article"
                },
                {
                    "date": "01/02/2024",
                    "featured": false,
                    "title": "Second Article",
                    "tags": ["Bridge"],
                    "slug": "second-article"
                }
            ]
        }"#;

        let content: ContentJson = serde_json::from_str(json).unwrap();
        assert_eq!(content.posts.len(), 2);
        assert_eq!(content.posts[0].title, "First Article");
        assert_eq!(content.posts[1].title, "Second Article");
        assert!(content.posts[0].rekt.is_some());
        assert!(content.posts[1].rekt.is_none());
    }

    #[test]
    fn test_json_article_roundtrip() {
        let original = create_test_article();
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: JsonArticle = serde_json::from_str(&json).unwrap();
        
        assert_eq!(original.date, deserialized.date);
        assert_eq!(original.title, deserialized.title);
        assert_eq!(original.slug, deserialized.slug);
        assert_eq!(original.featured, deserialized.featured);
        assert_eq!(original.tags, deserialized.tags);
        assert_eq!(original.excerpt, deserialized.excerpt);
        
        // Compare rekt details
        if let (Some(orig_rekt), Some(deser_rekt)) = (&original.rekt, &deserialized.rekt) {
            assert_eq!(orig_rekt.amount, deser_rekt.amount);
            assert_eq!(orig_rekt.audit, deser_rekt.audit);
            assert_eq!(orig_rekt.date, deser_rekt.date);
        }
    }

    #[test]
    fn test_invalid_json_handling() {
        let invalid_json = r#"{
            "date": "01/01/2024",
            "title": "Test",
            // Missing required slug field
        }"#;

        let result: Result<JsonArticle, _> = serde_json::from_str(invalid_json);
        assert!(result.is_err());
    }
} 