use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

use rayon::prelude::*;
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
        // Initialize stemmer and stopwords
        let stemmer = Stemmer::create(Algorithm::English);
        let stop_words_vec = stop_words::get(stop_words::LANGUAGE::English);

        // Additional domain-specific stopwords as Rekt News articles covers similar topics.
        let mut all_stop_words = stop_words_vec;
        all_stop_words.extend(vec![
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
        ]);

        let stop_words = all_stop_words.into_iter().collect();

        // Compile regex patterns
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
                all_terms.insert(term.clone());
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
    fn calculate_similarity_matrix(&mut self) {
        let total_docs = self.documents.len();

        // Initialize the matrix with zeros
        self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];

        // Pre-compute magnitudes for all documents
        let magnitudes: Vec<f32> = self
            .tfidf_matrix
            .iter()
            .map(|vec| vec.iter().map(|x| x * x).sum::<f32>().sqrt())
            .collect();

        // Vector of indices to parallelize over
        let indices: Vec<(usize, usize)> = (0..total_docs)
            .flat_map(|i| (i..total_docs).map(move |j| (i, j)))
            .collect();

        // Compute similarities in parallel and collect the results
        let similarities: Vec<(usize, usize, f32)> = indices
            .par_iter() // Parallel iterator
            .map(|&(i, j)| {
                let dot_product = self.tfidf_matrix[i]
                    .iter()
                    .zip(&self.tfidf_matrix[j])
                    .map(|(a, b)| a * b)
                    .sum::<f32>();

                let similarity = if magnitudes[i] > 0.0 && magnitudes[j] > 0.0 {
                    dot_product / (magnitudes[i] * magnitudes[j])
                } else {
                    0.0
                };

                (i, j, similarity)
            })
            .collect();

        // Fill the similarity matrix with the computed values
        for (i, j, similarity) in similarities {
            self.similarity_matrix[i][j] = similarity;
            if i != j {
                self.similarity_matrix[j][i] = similarity; // Mirror for symmetry
            }
        }
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

        // Remove stop words, apply stemming, and filter out invalid terms
        let processed_words: Vec<String> = words
            .into_iter()
            .filter(|word| !self.stop_words.contains(word) && self.is_valid_term(word))
            .map(|word| self.stemmer.stem(&word).to_string())
            .collect();

        // Count term frequencies
        let mut term_frequencies: HashMap<String, usize> = HashMap::new();
        for word in processed_words {
            *term_frequencies.entry(word).or_insert(0) += 1;
        }

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

        // Calculate IDF for each term
        for (term, count) in term_doc_count {
            let idf = (total_docs / (count as f32 + 1.0)).ln();
            self.idf_scores.insert(term, idf);
        }
    }

    // Calculate TF-IDF score for a term in a document
    fn calculate_tfidf(&self, term: &str, doc: &Document) -> f32 {
        let term_count = *doc.term_freq.get(term).unwrap_or(&0);

        if term_count == 0 {
            return 0.0;
        }

        let total_words: usize = doc.term_freq.values().sum();
        let tf = term_count as f32 / total_words as f32;
        let idf = self.idf_scores.get(term).unwrap_or(&0.0);

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
    let example_article = "wormhole-rekt";

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
