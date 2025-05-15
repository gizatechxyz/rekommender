use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

use luminal::prelude::*;
use luminal_cpu::CPUCompiler;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use stop_words;

struct Document {
    id: String,
    title: String,
    term_freq: HashMap<String, usize>,
}

struct RecommenderSystem {
    documents: Vec<Document>,
    stemmer: Stemmer,
    stop_words: HashSet<String>,
    idf_scores: HashMap<String, f32>,
    // Regex patterns for filtering irrelevant content
    url_pattern: Regex,
    crypto_address_pattern: Regex,
    twitter_handle_pattern: Regex,
    markdown_pattern: Regex,
    file_path_pattern: Regex,
    // Pre-computed data
    term_set: Vec<String>,            // All unique terms across all documents
    tfidf_matrix: Vec<Vec<f32>>,      // TF-IDF vectors for all documents
    similarity_matrix: Vec<Vec<f32>>, // Cosine similarity matrix
}

impl RecommenderSystem {
    fn new() -> Self {
        // Initialize with English stemmer and stopwords
        let stemmer = Stemmer::create(Algorithm::English);
        let mut stop_words_vec = stop_words::get(stop_words::LANGUAGE::English);

        // Add rekt-specific terms that are too common to be useful for recommendations
        let rekt_stop_words = vec![
            "eth".to_string(),
            "btc".to_string(),
            "defi".to_string(),
            "crypto".to_string(),
            "rekt".to_string(),
            "blockchain".to_string(),
            "token".to_string(),
            "tokens".to_string(),
            "https".to_string(),
            "http".to_string(),
            "com".to_string(),
            "www".to_string(),
            "twitter".to_string(),
            "tweet".to_string(),
            "status".to_string(),
            "attack".to_string(),
            "exploit".to_string(),
        ];

        stop_words_vec.extend(rekt_stop_words);

        let stop_words = stop_words_vec.into_iter().collect();

        // Compile regex patterns once for better performance
        let url_pattern = Regex::new(r"https?://\S+|www\.\S+").unwrap();
        let crypto_address_pattern =
            Regex::new(r"0x[a-fA-F0-9]{40}|[13][a-km-zA-HJ-NP-Z1-9]{25,34}").unwrap();
        let twitter_handle_pattern = Regex::new(r"@\w+").unwrap();
        let markdown_pattern = Regex::new(r"!\[.*?\]\(.*?\)|[*_#>`{}]|\[.*?\]\(.*?\)").unwrap();
        let file_path_pattern = Regex::new(r"(?i)\.(?:png|jpg|jpeg|gif|md|svg|pdf)").unwrap();

        RecommenderSystem {
            documents: Vec::new(),
            stemmer,
            stop_words,
            idf_scores: HashMap::new(),
            url_pattern,
            crypto_address_pattern,
            twitter_handle_pattern,
            markdown_pattern,
            file_path_pattern,
            term_set: Vec::new(),
            tfidf_matrix: Vec::new(),
            similarity_matrix: Vec::new(),
        }
    }

    // Load and process all markdown files from a directory
    fn load_documents(&mut self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let start_time = Instant::now();

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "md") {
                let content = fs::read_to_string(&path)?;
                let file_name = path.file_stem().unwrap().to_string_lossy().to_string();

                // Process title (from filename)
                let title = file_name.replace("-", " ");

                // Process the document content
                let term_freq = self.process_text(&content);

                self.documents.push(Document {
                    id: file_name,
                    title,
                    term_freq,
                });
            }
        }

        // Calculate IDF scores
        self.calculate_idf_scores();

        // Create the term set with all unique terms
        self.build_term_set();

        // Calculate TF-IDF matrix
        self.calculate_tfidf_matrix();

        // Calculate similarity matrix
        self.calculate_similarity_matrix();

        let duration = start_time.elapsed();
        println!(
            "Loaded and processed {} documents in {:.2?}",
            self.documents.len(),
            duration
        );
        Ok(())
    }

    // Create a set of all unique terms across all documents
    fn build_term_set(&mut self) {
        let mut all_terms = HashSet::new();

        for doc in &self.documents {
            for term in doc.term_freq.keys() {
                // Only include terms that have IDF scores (passed our filters)
                if self.idf_scores.contains_key(term) {
                    all_terms.insert(term.clone());
                }
            }
        }

        self.term_set = all_terms.into_iter().collect();
    }

    // Calculate TF-IDF matrix for all documents
    fn calculate_tfidf_matrix(&mut self) {
        let total_docs = self.documents.len();
        let total_terms = self.term_set.len();

        // Initialize the matrix with zeros
        self.tfidf_matrix = vec![vec![0.0; total_terms]; total_docs];

        // Fill the matrix with TF-IDF values
        for (doc_idx, doc) in self.documents.iter().enumerate() {
            for (term_idx, term) in self.term_set.iter().enumerate() {
                self.tfidf_matrix[doc_idx][term_idx] = self.calculate_tfidf(term, doc);
            }
        }
    }

    // Compute document magnitudes once and store them
    // We'll implement the standard cosine similarity formula:
    // cos(A,B) = (A·B) / (||A|| * ||B||)
    fn calculate_similarity_matrix(&mut self) {
        let start_time = Instant::now();
        let total_docs = self.documents.len();

        // Initialize a new Luminal graph
        let mut graph = Graph::new();

        // Convert TF-IDF matrix to Vec<f32> for Luminal
        let tfidf_data: Vec<f32> = self
            .tfidf_matrix
            .iter()
            .flat_map(|row| row.iter().map(|&val| val as f32))
            .collect();

        let total_terms = self.term_set.len();

        // Create a tensor for the TF-IDF matrix
        let tfidf_tensor = graph.tensor((total_docs, total_terms)).set(tfidf_data);

        // L2 Normalize each document vector before computing similarity
        // Square each element in the TF-IDF matrix
        let squared = tfidf_tensor.clone() * tfidf_tensor.clone();

        // Sum along the terms dimension (axis 1)
        let sum_squared = squared.sum_reduce(1);

        // Take square root to get the magnitude
        let magnitudes = sum_squared.sqrt() + 1e-8; // Add small epsilon to avoid division by zero

        // Normalize TF-IDF matrix by dividing each row by its magnitude
        let normalized_tfidf = tfidf_tensor / magnitudes.expand(1, total_terms);

        //  Compute all pairwise dot products using matrix multiplication
        // Since vectors are normalized, dot product equals cosine similarity
        let similarities = normalized_tfidf.matmul(normalized_tfidf.permute((1, 0)));

        // Mark the similarities tensor to be retrieved after graph execution
        let mut result = similarities.retrieve();

        // Compile the graph with CPU compiler
        graph.compile(<(GenericCompiler, CPUCompiler)>::default(), &mut result);

        // Execute the graph
        graph.execute_debug();

        // Get the results
        let similarity_data = result.data();

        // Convert results to the similarity matrix format
        self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];
        for i in 0..total_docs {
            for j in 0..total_docs {
                self.similarity_matrix[i][j] = similarity_data[i * total_docs + j];
            }
        }

        // Normalize self-similarity to exactly 1.0 (floating point precision issues)
        for i in 0..total_docs {
            self.similarity_matrix[i][i] = 1.0;
        }

        let duration = start_time.elapsed();
        println!(
            "Similarity matrix calculated using Luminal in {:.2?}",
            duration
        );
    }

    // Clean text by removing irrelevant elements
    fn clean_text(&self, text: &str) -> String {
        // Extract markdown content only
        let content = if text.starts_with("---") {
            if let Some(end_index) = text[3..].find("---") {
                &text[(end_index + 6)..]
            } else {
                text
            }
        } else {
            text
        };

        // Remove URLs with
        let text = self.url_pattern.replace_all(content, " ");

        // Remove crypto addresses
        let text = self.crypto_address_pattern.replace_all(&text, " ");

        // Remove Twitter handles
        let text = self.twitter_handle_pattern.replace_all(&text, " ");

        // Remove markdown formatting
        let text = self.markdown_pattern.replace_all(&text, " ");

        // Remove file paths/references
        let text = self.file_path_pattern.replace_all(&text, " ");

        // Remove special characters
        let mut result = String::with_capacity(text.len());
        for c in text.chars() {
            match c {
                '|' | '#' | '*' | '_' | '>' | '-' | '"' | '\'' | ':' | ';' | '…' | '(' | ')'
                | '[' | ']' | '{' | '}' | '/' | '\\' => {
                    result.push(' ');
                }
                _ => {
                    result.push(c);
                }
            }
        }

        // Remove extra whitespace
        let text = result.split_whitespace().collect::<Vec<&str>>().join(" ");

        text
    }

    // Filter out numeric strings and very short terms
    fn is_valid_term(&self, term: &str) -> bool {
        if term.len() <= 2 {
            return false;
        }

        // Skip purely numeric terms
        if term.chars().all(|c| c.is_digit(10)) {
            return false;
        }

        // Skip terms that are mostly numbers
        let digit_count = term.chars().filter(|c| c.is_digit(10)).count();
        if digit_count > term.len() / 2 {
            return false;
        }

        true
    }

    // Process text: tokenize, remove stop words, stem, and count frequencies
    fn process_text(&self, text: &str) -> HashMap<String, usize> {
        // Clean the text
        let cleaned_text = self.clean_text(text);
        let lower_text = cleaned_text.to_lowercase();

        // Simple tokenization
        let words: Vec<String> = lower_text
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        // Apply stemmming, remove stop words, apply stemming, and filter out invalid terms
        let processed_words: Vec<String> = words
            .into_iter()
            .filter(|word| self.is_valid_term(word))
            .map(|word| self.stemmer.stem(&word).to_string())
            .filter(|stemmed_word| !self.stop_words.contains(stemmed_word))
            .collect();

        // Count term frequencies
        let mut term_frequencies: HashMap<String, usize> = HashMap::new();
        for word in processed_words {
            *term_frequencies.entry(word).or_insert(0) += 1;
        }

        // Filter out extremely rare terms that appear only once
        term_frequencies.retain(|_, &mut count| count > 1);

        term_frequencies
    }

    // Calculate IDF scores for all terms in the corpus
    fn calculate_idf_scores(&mut self) {
        let total_docs = self.documents.len() as f32;
        let mut term_doc_count: HashMap<String, usize> = HashMap::new();

        // Count documents containing each term
        for doc in &self.documents {
            for term in doc.term_freq.keys() {
                *term_doc_count.entry(term.clone()).or_insert(0) += 1;
            }
        }

        // Filter terms that appear in too few or too many documents
        let min_doc_threshold = 2; // Appear in at least 2 documents
        let max_doc_percentage = 0.70; // Don't appear in more than 70% of documents
        let max_doc_threshold = (total_docs * max_doc_percentage) as usize;

        // Calculate IDF with a smoother formula and apply thresholds
        for (term, count) in term_doc_count {
            if count >= min_doc_threshold && count <= max_doc_threshold {
                // log(N/(1+df)) + 1 instead of log(N/df) should give more weight to important terms.
                let idf = (total_docs / (count as f32 + 1.0)).ln() + 1.0;
                self.idf_scores.insert(term, idf);
            }
        }
    }

    // Calculate TF-IDF score for a term in a document
    fn calculate_tfidf(&self, term: &str, doc: &Document) -> f32 {
        // Only calculate for terms with IDF scores
        if !self.idf_scores.contains_key(term) {
            return 0.0;
        }

        let term_count = *doc.term_freq.get(term).unwrap_or(&0);

        if term_count == 0 {
            return 0.0;
        }

        // Calculate term frequency using augmented frequency formula
        // TF = 0.5 + 0.5 * (term_count / max_term_count)
        // This prevents bias towards longer documents
        let max_count = doc.term_freq.values().max().unwrap_or(&1);
        let tf = 0.5 + 0.5 * (term_count as f32 / *max_count as f32);

        // Get IDF score
        let idf = self.idf_scores.get(term).unwrap_or(&0.0);

        // TF-IDF
        tf * idf
    }

    // Get recommendations for a document by ID using the pre-computed similarity matrix
    fn get_recommendations(&self, doc_id: &str, num_recommendations: usize) -> Vec<(String, f32)> {
        // Find the specified document
        let doc_idx = self.documents.iter().position(|d| d.id == doc_id);

        if let Some(idx) = doc_idx {
            // Get the similarities from the pre-computed matrix
            let similarities: Vec<(usize, f32)> = self.similarity_matrix[idx]
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx) // Filter out the document itself
                .map(|(i, &score)| (i, score))
                .collect();

            // Sort by similarity score (descending)
            let mut sorted_similarities = similarities;
            sorted_similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Return top N recommendations
            sorted_similarities
                .into_iter()
                .take(num_recommendations)
                .map(|(doc_idx, score)| (self.documents[doc_idx].id.clone(), score))
                .collect()
        } else {
            println!("Document with ID '{}' not found", doc_id);
            Vec::new()
        }
    }

    // Print details about a document by ID
    fn print_document_details(&self, doc_id: &str) {
        if let Some(doc) = self.documents.iter().find(|d| d.id == doc_id) {
            println!("Document: {}", doc.title);

            // Get the top 10 most frequent terms
            let mut terms: Vec<(&String, &usize)> = doc.term_freq.iter().collect();
            terms.sort_by(|a, b| b.1.cmp(a.1));

            println!("Top 10 terms:");
            for (i, (term, freq)) in terms.iter().take(10).enumerate() {
                println!("  {}. {} ({})", i + 1, term, freq);
            }

            println!();
        } else {
            println!("Document with ID '{}' not found", doc_id);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut recommender = RecommenderSystem::new();

    // Load documents from the rekt_articles directory
    let articles_path = Path::new("rekt_articles");
    recommender.load_documents(articles_path)?;

    // Example usage: Get recommendations for a specific article
    let example_article = "alexlab-rekt";

    println!("Details for '{}':", example_article);
    recommender.print_document_details(example_article);

    println!("Top 5 recommendations for '{}':", example_article);
    let recommendations = recommender.get_recommendations(example_article, 5);

    for (i, (article_id, score)) in recommendations.iter().enumerate() {
        println!("  {}. {} (similarity: {:.4})", i + 1, article_id, score);
        recommender.print_document_details(article_id);
    }

    Ok(())
}
