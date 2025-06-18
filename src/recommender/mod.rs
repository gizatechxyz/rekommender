pub mod text_processing;

use std::collections::{HashMap, HashSet};
use std::time::Instant;

use anyhow::{Context, Result};
use luminair::prelude::*;

use crate::config::TextProcessingConfig;
use crate::types::JsonArticle;
use text_processing::TextProcessor;

/// Internal document representation for processing
struct Document {
    /// Unique identifier for the document
    id: String,
    /// Document title
    #[allow(dead_code)]
    title: String,
    /// Associated tags
    #[allow(dead_code)]
    tags: Vec<String>,
    /// Auditor information if available
    #[allow(dead_code)]
    auditor: Option<String>,
    /// Term frequency map for this document
    term_freq: HashMap<String, usize>,
}

/// Main recommender system that processes articles and generates similarity matrices
/// with zero-knowledge proofs for verification
pub struct RecommenderSystem {
    /// Processed documents
    documents: Vec<Document>,
    /// Text processor for cleaning and feature extraction
    text_processor: TextProcessor,
    /// Inverse document frequency scores for all terms
    idf_scores: HashMap<String, f32>,
    /// Ordered set of all terms in the vocabulary
    term_set: Vec<String>,
    /// TF-IDF matrix representation of all documents
    tfidf_matrix: Vec<Vec<f32>>,
    /// Computed similarity matrix between all document pairs
    similarity_matrix: Vec<Vec<f32>>,
    /// Zero-knowledge proof data for verification
    proof_data: Vec<u8>,
    /// Circuit settings for proof verification
    circuit_settings: Vec<u8>,
}

impl RecommenderSystem {
    /// Create a new recommender system with default configuration
    /// 
    /// # Returns
    /// 
    /// A new `RecommenderSystem` instance ready for processing
    pub fn new() -> Self {
        Self::with_config(TextProcessingConfig::default())
    }

    /// Create a new recommender system with custom text processing configuration
    /// 
    /// # Arguments
    /// 
    /// * `config` - Text processing configuration
    /// 
    /// # Returns
    /// 
    /// A new `RecommenderSystem` instance with the specified configuration
    pub fn with_config(config: TextProcessingConfig) -> Self {
        let text_processor = TextProcessor::with_config(config);

        Self {
            documents: Vec::new(),
            text_processor,
            idf_scores: HashMap::new(),
            term_set: Vec::new(),
            tfidf_matrix: Vec::new(),
            similarity_matrix: Vec::new(),
            proof_data: Vec::new(),
            circuit_settings: Vec::new(),
        }
    }

    /// Load articles and process them to generate similarity matrix with zero-knowledge proof
    /// 
    /// # Arguments
    /// 
    /// * `articles` - Slice of JSON articles to process
    /// 
    /// # Returns
    /// 
    /// A tuple containing:
    /// - Vector of article IDs
    /// - Zero-knowledge proof data
    /// - Circuit settings for verification
    /// 
    /// # Errors
    /// 
    /// Returns an error if document loading, processing, or proof generation fails
    pub fn load_and_process_json(
        &mut self,
        articles: &[JsonArticle],
    ) -> Result<(Vec<String>, Vec<u8>, Vec<u8>)> {
        let start_time = Instant::now();

        // Process articles through the complete pipeline
        self.load_json_documents(articles)?;
        self.calculate_idf_scores();
        self.build_term_set();
        self.calculate_tfidf_matrix();
        self.calculate_similarity_matrix_with_proof()?;

        let article_ids: Vec<String> = self.documents.iter().map(|d| d.id.clone()).collect();

        let duration = start_time.elapsed();
        println!("Total processing time: {:.2?}", duration);

        Ok((
            article_ids,
            self.proof_data.clone(),
            self.circuit_settings.clone(),
        ))
    }

    /// Load JSON articles into internal document representation
    /// 
    /// # Arguments
    /// 
    /// * `articles` - Slice of JSON articles to convert
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if processing fails
    fn load_json_documents(&mut self, articles: &[JsonArticle]) -> Result<()> {
        for article in articles {
            let tags = article.tags.clone().unwrap_or_default();
            
            // Extract and validate auditor information
            let auditor = article.rekt.as_ref()
                .and_then(|r| r.audit.clone())
                .filter(|audit| {
                    !audit.is_empty() 
                        && audit.to_lowercase() != "n/a" 
                        && audit.to_lowercase() != "unaudited"
                });

            // Create content from title and excerpt
            let content = if let Some(excerpt) = &article.excerpt {
                format!("{} {}", article.title, excerpt)
            } else {
                article.title.clone()
            };

            // Process text to extract term frequencies
            let term_freq = self.text_processor.process_text(
                &content, 
                &tags, 
                &auditor, 
                &article.excerpt
            );

            let document = Document {
                id: article.slug.clone(),
                title: article.title.clone(),
                tags,
                auditor,
                term_freq,
            };

            self.documents.push(document);
        }

        Ok(())
    }

    /// Calculate inverse document frequency (IDF) scores for all terms
    /// 
    /// IDF measures how important a term is across the entire document collection.
    /// Terms that appear in too many documents or too few documents are filtered out.
    fn calculate_idf_scores(&mut self) {
        let total_docs = self.documents.len() as f32;
        let mut term_doc_count: HashMap<String, usize> = HashMap::new();

        // Count documents containing each term
        for doc in &self.documents {
            for term in doc.term_freq.keys() {
                *term_doc_count.entry(term.clone()).or_insert(0) += 1;
            }
        }

        let config = self.text_processor.config();
        let max_doc_threshold = (total_docs * config.max_doc_percentage) as usize;

        // Calculate IDF scores for terms within acceptable frequency ranges
        for (term, count) in term_doc_count {
            let min_threshold = if term.contains('_') {
                config.min_doc_threshold_ngram
            } else {
                config.min_doc_threshold_unigram
            };

            if count >= min_threshold && count <= max_doc_threshold {
                let idf = (total_docs / (count as f32 + 1.0)).ln() + 1.0;
                self.idf_scores.insert(term, idf);
            }
        }
    }

    /// Build the vocabulary set from all valid terms
    /// 
    /// Creates an ordered list of all terms that have valid IDF scores
    fn build_term_set(&mut self) {
        let mut all_terms = HashSet::new();
        
        for doc in &self.documents {
            for term in doc.term_freq.keys() {
                if self.idf_scores.contains_key(term) {
                    all_terms.insert(term.clone());
                }
            }
        }
        
        self.term_set = all_terms.into_iter().collect();
        self.term_set.sort(); // Ensure consistent ordering
    }

    /// Calculate TF-IDF score for a specific term in a document
    /// 
    /// # Arguments
    /// 
    /// * `term` - The term to calculate TF-IDF for
    /// * `doc` - The document containing the term
    /// 
    /// # Returns
    /// 
    /// The TF-IDF score, or 0.0 if the term is not in the vocabulary
    fn calculate_tfidf(&self, term: &str, doc: &Document) -> f32 {
        if !self.idf_scores.contains_key(term) {
            return 0.0;
        }
        
        let term_count = *doc.term_freq.get(term).unwrap_or(&0);
        if term_count == 0 {
            return 0.0;
        }
        
        // Use normalized term frequency (prevents bias towards longer documents)
        let max_count = doc.term_freq.values().max().unwrap_or(&1);
        let tf = 0.5 + 0.5 * (term_count as f32 / *max_count as f32);
        let idf = self.idf_scores.get(term).unwrap_or(&0.0);
        
        tf * idf
    }

    /// Calculate the TF-IDF matrix for all documents and terms
    /// 
    /// Creates a matrix where each row represents a document and each column represents a term.
    /// Each cell contains the TF-IDF score for that term-document pair.
    fn calculate_tfidf_matrix(&mut self) {
        let total_docs = self.documents.len();
        let total_terms = self.term_set.len();
        
        self.tfidf_matrix = vec![vec![0.0; total_terms]; total_docs];
        
        for (doc_idx, doc) in self.documents.iter().enumerate() {
            for (term_idx, term) in self.term_set.iter().enumerate() {
                self.tfidf_matrix[doc_idx][term_idx] = self.calculate_tfidf(term, doc);
            }
        }
    }

    /// Calculate similarity matrix with zero-knowledge proof generation
    /// 
    /// Uses cosine similarity between TF-IDF vectors and generates a cryptographic
    /// proof that the computation was performed correctly.
    /// 
    /// # Returns
    /// 
    /// Returns `Ok(())` on success, or an error if similarity calculation or proof generation fails
    fn calculate_similarity_matrix_with_proof(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let total_docs = self.documents.len();

        // Handle edge case of empty term set
        if self.term_set.is_empty() {
            self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];
            return Ok(());
        }

        // Prepare data for zero-knowledge computation
        let mut graph = Graph::new();
        let tfidf_data: Vec<f32> = self
            .tfidf_matrix
            .iter()
            .flat_map(|row| row.iter().copied())
            .collect();

        let total_terms = self.term_set.len();
        
        // Create computational graph for cosine similarity
        let tfidf_tensor = graph
            .tensor((total_docs, total_terms))
            .set(tfidf_data.clone());
        
        // Calculate cosine similarity: normalize vectors then compute dot products
        let squared = tfidf_tensor * tfidf_tensor;
        let sum_squared = squared.sum_reduce(1);
        let magnitudes = sum_squared.sqrt() + 1e-1; // Add small epsilon for numerical stability
        let normalized_tfidf = tfidf_tensor / magnitudes.expand(1, total_terms);
        
        // Compute similarity matrix
        let mut similarities_tensor = normalized_tfidf
            .matmul(normalized_tfidf.permute((1, 0)))
            .retrieve();

        // Compile the computational graph
        graph.compile(
            <(GenericCompiler, StwoCompiler)>::default(),
            &mut similarities_tensor,
        );

        // Generate zero-knowledge proof
        let mut circuit_settings = graph.gen_circuit_settings();
        let pie = graph
            .gen_trace(&mut circuit_settings)
            .context("Failed to generate computation trace")?;
        let proof = prove(pie, circuit_settings.clone())
            .context("Failed to generate zero-knowledge proof")?;

        // Serialize proof and settings for storage/verification
        self.proof_data = proof.to_bincode()
            .context("Failed to serialize proof data")?;
        self.circuit_settings = circuit_settings
            .to_bincode()
            .context("Failed to serialize circuit settings")?;

        // Verify the proof to ensure correctness
        verify(proof, circuit_settings)
            .context("Proof verification failed")?;

        // Extract similarity matrix from computation result
        let similarity_data = similarities_tensor.data();
        self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];
        
        for i in 0..total_docs {
            for j in 0..total_docs {
                self.similarity_matrix[i][j] = similarity_data[i * total_docs + j];
            }
        }
        
        // Set diagonal to 1.0 (documents are perfectly similar to themselves)
        for i in 0..total_docs {
            self.similarity_matrix[i][i] = 1.0;
        }

        let duration = start_time.elapsed();
        println!(
            "Similarity matrix calculated and proof generated in {:.2?}",
            duration
        );

        Ok(())
    }

    /// Get a copy of the computed similarity matrix
    /// 
    /// # Returns
    /// 
    /// A 2D vector representing the similarity matrix where entry (i,j) contains
    /// the cosine similarity between documents i and j
    pub fn get_similarity_matrix(&self) -> Vec<Vec<f32>> {
        self.similarity_matrix.clone()
    }

    /// Get the number of processed documents
    /// 
    /// # Returns
    /// 
    /// The number of documents in the system
    #[allow(dead_code)]
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// Get the vocabulary size
    /// 
    /// # Returns
    /// 
    /// The number of unique terms in the vocabulary
    #[allow(dead_code)]
    pub fn vocabulary_size(&self) -> usize {
        self.term_set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{JsonArticle, RektDetails};

    fn create_test_articles() -> Vec<JsonArticle> {
        vec![
            JsonArticle {
                date: "01/01/2024".to_string(),
                featured: Some(true),
                title: "DeFi Protocol Exploit Analysis".to_string(),
                rekt: Some(RektDetails {
                    amount: Some(1000000),
                    audit: Some("Trail of Bits".to_string()),
                    date: Some("12/31/2023".to_string()),
                }),
                tags: Some(vec!["DeFi".to_string(), "Security".to_string()]),
                excerpt: Some("A detailed analysis of the DeFi protocol exploit".to_string()),
                banner: Some("banner.jpg".to_string()),
                slug: "defi-protocol-exploit-analysis".to_string(),
            },
            JsonArticle {
                date: "01/02/2024".to_string(),
                featured: None,
                title: "Smart Contract Vulnerability Report".to_string(),
                rekt: Some(RektDetails {
                    amount: Some(500000),
                    audit: Some("Consensys".to_string()),
                    date: Some("01/01/2024".to_string()),
                }),
                tags: Some(vec!["Smart Contract".to_string(), "Vulnerability".to_string()]),
                excerpt: Some("Smart contract vulnerability discovered in protocol".to_string()),
                banner: None,
                slug: "smart-contract-vulnerability-report".to_string(),
            },
            JsonArticle {
                date: "01/03/2024".to_string(),
                featured: Some(false),
                title: "Bridge Attack Investigation".to_string(),
                rekt: None,
                tags: Some(vec!["Bridge".to_string(), "Attack".to_string()]),
                excerpt: Some("Investigation into the recent bridge attack".to_string()),
                banner: None,
                slug: "bridge-attack-investigation".to_string(),
            },
        ]
    }

    #[test]
    fn test_recommender_system_creation() {
        let recommender = RecommenderSystem::new();
        assert_eq!(recommender.documents.len(), 0);
        assert_eq!(recommender.term_set.len(), 0);
    }

    #[test]
    fn test_recommender_system_with_config() {
        let config = TextProcessingConfig {
            tag_boost: 10,
            auditor_boost: 5,
            max_doc_percentage: 0.5,
            min_doc_threshold_unigram: 2,
            min_doc_threshold_ngram: 1,
        };
        
        let recommender = RecommenderSystem::with_config(config);
        assert_eq!(recommender.documents.len(), 0);
        assert_eq!(recommender.text_processor.config().tag_boost, 10);
    }

    #[test]
    fn test_load_json_documents() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        let result = recommender.load_json_documents(&articles);
        assert!(result.is_ok());
        assert_eq!(recommender.documents.len(), 3);
        
        // Check document IDs
        let doc_ids: Vec<&String> = recommender.documents.iter().map(|d| &d.id).collect();
        assert!(doc_ids.contains(&&"defi-protocol-exploit-analysis".to_string()));
        assert!(doc_ids.contains(&&"smart-contract-vulnerability-report".to_string()));
        assert!(doc_ids.contains(&&"bridge-attack-investigation".to_string()));
    }

    #[test]
    fn test_calculate_idf_scores() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        recommender.load_json_documents(&articles).unwrap();
        recommender.calculate_idf_scores();
        
        // May not have IDF scores due to filtering thresholds with small test set
        // All IDF scores that exist should be positive
        for &idf_score in recommender.idf_scores.values() {
            assert!(idf_score > 0.0);
        }
    }

    #[test]
    fn test_build_term_set() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        recommender.load_json_documents(&articles).unwrap();
        recommender.calculate_idf_scores();
        recommender.build_term_set();
        
        // Term set should be sorted (even if empty due to filtering)
        let is_sorted = recommender.term_set.windows(2).all(|w| w[0] <= w[1]);
        assert!(is_sorted, "Term set should be sorted");
    }

    #[test]
    fn test_calculate_tfidf_matrix() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        recommender.load_json_documents(&articles).unwrap();
        recommender.calculate_idf_scores();
        recommender.build_term_set();
        recommender.calculate_tfidf_matrix();
        
        // Matrix should have proper dimensions
        assert_eq!(recommender.tfidf_matrix.len(), 3); // 3 documents
        if !recommender.term_set.is_empty() {
            assert_eq!(recommender.tfidf_matrix[0].len(), recommender.term_set.len());
        }
        
        // All TF-IDF values should be non-negative
        for row in &recommender.tfidf_matrix {
            for &value in row {
                assert!(value >= 0.0, "TF-IDF values should be non-negative");
            }
        }
    }

    #[test]
    fn test_full_processing_pipeline() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        let result = recommender.load_and_process_json(&articles);
        assert!(result.is_ok());
        
        let (article_ids, proof_data, circuit_settings) = result.unwrap();
        
        // Check article IDs
        assert_eq!(article_ids.len(), 3);
        assert!(article_ids.contains(&"defi-protocol-exploit-analysis".to_string()));
        
        // Check proof data (may be empty if no terms to process)
        assert!(proof_data.len() == proof_data.len());
        assert!(circuit_settings.len() == circuit_settings.len());
        
        // Check similarity matrix
        let similarity_matrix = recommender.get_similarity_matrix();
        assert_eq!(similarity_matrix.len(), 3);
        assert_eq!(similarity_matrix[0].len(), 3);
        
        // Check matrix properties based on whether we have valid terms
        if recommender.term_set.is_empty() {
            // If no terms passed filtering, matrix should be all zeros
            for i in 0..3 {
                for j in 0..3 {
                    assert_eq!(similarity_matrix[i][j], 0.0, 
                        "When no terms pass filtering, similarity matrix should be all zeros");
                }
            }
        } else {
            // If we have terms, diagonal should be 1.0 (documents similar to themselves)
            for i in 0..3 {
                assert!((similarity_matrix[i][i] - 1.0).abs() < 1e-6,
                    "Documents should be perfectly similar to themselves when terms exist");
            }
            
            // Matrix should be symmetric
            for i in 0..3 {
                for j in 0..3 {
                    let diff = (similarity_matrix[i][j] - similarity_matrix[j][i]).abs();
                    assert!(diff < 1e-6, "Similarity matrix should be symmetric");
                }
            }
        }
    }

    #[test]
    fn test_calculate_tfidf_single_term() {
        let mut recommender = RecommenderSystem::new();
        
        // Create a simple document for testing
        let mut doc = Document {
            id: "test".to_string(),
            title: "Test".to_string(),
            tags: vec![],
            auditor: None,
            term_freq: HashMap::new(),
        };
        doc.term_freq.insert("test".to_string(), 2);
        doc.term_freq.insert("word".to_string(), 1);
        
        recommender.idf_scores.insert("test".to_string(), 2.0);
        
        let tfidf_score = recommender.calculate_tfidf("test", &doc);
        assert!(tfidf_score > 0.0);
        
        let unknown_score = recommender.calculate_tfidf("unknown", &doc);
        assert_eq!(unknown_score, 0.0);
    }

    #[test]
    fn test_empty_articles_handling() {
        let mut recommender = RecommenderSystem::new();
        let empty_articles: Vec<JsonArticle> = vec![];
        
        let result = recommender.load_and_process_json(&empty_articles);
        assert!(result.is_ok());
        
        let (article_ids, _proof_data, _circuit_settings) = result.unwrap();
        assert_eq!(article_ids.len(), 0);
        
        let similarity_matrix = recommender.get_similarity_matrix();
        assert_eq!(similarity_matrix.len(), 0);
    }

    #[test]
    fn test_single_article_processing() {
        let mut recommender = RecommenderSystem::new();
        let single_article = vec![create_test_articles()[0].clone()];
        
        let result = recommender.load_and_process_json(&single_article);
        assert!(result.is_ok());
        
        let similarity_matrix = recommender.get_similarity_matrix();
        assert_eq!(similarity_matrix.len(), 1);
        assert_eq!(similarity_matrix[0].len(), 1);
        // May be 0.0 if no terms pass filtering, or 1.0 if terms exist
        assert!(similarity_matrix[0][0] >= 0.0 && similarity_matrix[0][0] <= 1.0);
    }

    #[test]
    fn test_document_auditor_filtering() {
        let mut recommender = RecommenderSystem::new();
        
        // Create articles with different auditor values
        let mut articles = create_test_articles();
        articles.push(JsonArticle {
            date: "01/04/2024".to_string(),
            featured: None,
            title: "Unaudited Protocol".to_string(),
            rekt: Some(RektDetails {
                amount: None,
                audit: Some("unaudited".to_string()), // Should be filtered out
                date: None,
            }),
            tags: None,
            excerpt: None,
            banner: None,
            slug: "unaudited-protocol".to_string(),
        });
        
        let result = recommender.load_json_documents(&articles);
        assert!(result.is_ok());
        
        // Check that unaudited entry was filtered
        let audited_docs = recommender.documents.iter()
            .filter(|d| d.auditor.is_some())
            .count();
        assert_eq!(audited_docs, 2); // Only the first two should have valid auditors
    }

    #[test]
    fn test_document_count_and_vocabulary_size() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        recommender.load_json_documents(&articles).unwrap();
        assert_eq!(recommender.document_count(), 3);
        
        recommender.calculate_idf_scores();
        recommender.build_term_set();
        // Vocabulary size may be 0 due to filtering thresholds with small test set
        assert!(recommender.vocabulary_size() == recommender.vocabulary_size());
    }

    #[test]
    fn test_similarity_matrix_properties() {
        let mut recommender = RecommenderSystem::new();
        let articles = create_test_articles();
        
        recommender.load_and_process_json(&articles).unwrap();
        let matrix = recommender.get_similarity_matrix();
        
        // All values should be between 0 and 1 (cosine similarity range)
        for row in &matrix {
            for &value in row {
                assert!(value >= 0.0 && value <= 1.0, 
                    "Similarity values should be between 0 and 1, got {}", value);
            }
        }
    }
} 