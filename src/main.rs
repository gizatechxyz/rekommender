use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

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
    idf_scores: HashMap<String, f64>,
    // Regex patterns for filtering irrelevant content
    url_pattern: Regex,
    crypto_address_pattern: Regex,
    twitter_handle_pattern: Regex,
    markdown_pattern: Regex,
    file_path_pattern: Regex,
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

        let duration = start_time.elapsed();
        println!(
            "Loaded and processed {} documents in {:.2?}",
            self.documents.len(),
            duration
        );
        Ok(())
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
        let total_docs = self.documents.len() as f64;
        let mut term_doc_count: HashMap<String, usize> = HashMap::new();

        // Count documents containing each term
        for doc in &self.documents {
            for term in doc.term_freq.keys() {
                *term_doc_count.entry(term.clone()).or_insert(0) += 1;
            }
        }

        // Calculate IDF for each term
        for (term, count) in term_doc_count {
            let idf = (total_docs / (count as f64 + 1.0)).ln();
            self.idf_scores.insert(term, idf);
        }
    }

    // Calculate TF-IDF score for a term in a document
    fn calculate_tfidf(&self, term: &str, doc: &Document) -> f64 {
        let term_count = *doc.term_freq.get(term).unwrap_or(&0);

        if term_count == 0 {
            return 0.0;
        }

        let total_words: usize = doc.term_freq.values().sum();
        let tf = term_count as f64 / total_words as f64;
        let idf = self.idf_scores.get(term).unwrap_or(&0.0);

        tf * idf
    }

    // Get recommendations for a document by ID
    fn get_recommendations(&self, doc_id: &str, num_recommendations: usize) -> Vec<(String, f64)> {
        // Find the specified document
        let doc_idx = self.documents.iter().position(|d| d.id == doc_id);

        if let Some(idx) = doc_idx {
            let target_doc = &self.documents[idx];

            // Calculate similarity scores with all other documents
            let mut similarities: Vec<(String, f64)> = self
                .documents
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx) // Exclude the target document
                .map(|(_, doc)| {
                    // Calculate cosine similarity
                    let score = self.calculate_similarity(target_doc, doc);
                    (doc.id.clone(), score)
                })
                .collect();

            // Sort by similarity score (descending)
            similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            // Return top N recommendations
            similarities.into_iter().take(num_recommendations).collect()
        } else {
            println!("Document with ID '{}' not found", doc_id);
            Vec::new()
        }
    }

    // Calculate similarity between two documents using TF-IDF and cosine similarity
    fn calculate_similarity(&self, doc1: &Document, doc2: &Document) -> f64 {
        // Get unique terms from both documents
        let mut all_terms: HashSet<String> = HashSet::new();

        for term in doc1.term_freq.keys() {
            all_terms.insert(term.clone());
        }

        for term in doc2.term_freq.keys() {
            all_terms.insert(term.clone());
        }

        // Calculate TF-IDF vectors
        let mut vec1: Vec<f64> = Vec::with_capacity(all_terms.len());
        let mut vec2: Vec<f64> = Vec::with_capacity(all_terms.len());

        for term in &all_terms {
            vec1.push(self.calculate_tfidf(term, doc1));
            vec2.push(self.calculate_tfidf(term, doc2));
        }

        // Calculate cosine similarity
        self.cosine_similarity(&vec1, &vec2)
    }

    // Compute cosine similarity between two vectors
    fn cosine_similarity(&self, vec1: &[f64], vec2: &[f64]) -> f64 {
        let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(a, b)| a * b).sum();

        let magnitude1: f64 = vec1.iter().map(|x| x * x).sum::<f64>().sqrt();
        let magnitude2: f64 = vec2.iter().map(|x| x * x).sum::<f64>().sqrt();

        if magnitude1 > 0.0 && magnitude2 > 0.0 {
            dot_product / (magnitude1 * magnitude2)
        } else {
            0.0
        }
    }

    // Print details about a document by ID
    fn print_document_details(&self, doc_id: &str) {
        if let Some(doc) = self.documents.iter().find(|d| d.id == doc_id) {
            println!("Document: {}", doc.title);

            // Get the top 10 most frequent terms
            let mut terms: Vec<(&String, &usize)> = doc.term_freq.iter().collect();
            terms.sort_by(|a, b| b.1.cmp(a.1));

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
    let example_article = "airdrop-hunters";

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
