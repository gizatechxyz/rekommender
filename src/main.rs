use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Instant;

use luminair_graph::graph::LuminairGraph;
use luminair_graph::StwoCompiler;
use luminal::prelude::*;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use serde::Deserialize;
use stop_words;

// Recommender weights
const TAG_BOOST: usize = 5;
const AUDITOR_BOOST: usize = 3;
const MAX_DOC_PERCENTAGE: f32 = 0.80;
const MIN_DOC_THRESHOLD_UNIGRAM: usize = 5; // Unigrams must appear in at least 5 docs
const MIN_DOC_THRESHOLD_NGRAM: usize = 2;   // N-grams can be rarer, in at least 2 docs

#[derive(Debug, Deserialize)]
struct Frontmatter {
    title: String,
    tags: Option<Vec<String>>,
    rekt: Option<RektDetails>,
    excerpt: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RektDetails {
    audit: Option<String>,
}

struct Document {
    id: String, // Filename without .md
    title: String,
    tags: Vec<String>,
    auditor: Option<String>,
    term_freq: HashMap<String, usize>,
}

struct RecommenderSystem {
    documents: Vec<Document>,
    stemmer: Stemmer,
    stop_words: HashSet<String>,
    idf_scores: HashMap<String, f32>,
    url_pattern: Regex,
    crypto_address_pattern: Regex,
    twitter_handle_pattern: Regex,
    markdown_pattern: Regex,
    file_path_pattern: Regex,
    term_set: Vec<String>,
    tfidf_matrix: Vec<Vec<f32>>,
    similarity_matrix: Vec<Vec<f32>>,
}

impl RecommenderSystem {
    fn new() -> Self {
        let stemmer = Stemmer::create(Algorithm::English);
        let mut stop_words_vec = stop_words::get(stop_words::LANGUAGE::English);

        let rekt_stop_words_originals = vec![
            "eth", "btc", "defi", "crypto", "rekt", "blockchain", 
            "token", "tokens", "https", "http", "com", "www", 
            "twitter", "tweet", "status", "attack", "exploit",
        ];
        
        // Add original custom stop words
        stop_words_vec.extend(rekt_stop_words_originals.iter().map(|s| s.to_string()));
        
        // Add stemmed versions of these custom stop words
        let stemmed_rekt_stop_words: Vec<String> = rekt_stop_words_originals
            .iter()
            .map(|word| stemmer.stem(word).to_string())
            .collect();
        stop_words_vec.extend(stemmed_rekt_stop_words);

        let stop_words = stop_words_vec.into_iter().collect();

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

    fn load_documents(&mut self, directory: &Path) -> Result<(), Box<dyn Error>> {
        let start_time = Instant::now();

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let full_content = fs::read_to_string(&path)?;
                let file_name_id = path.file_stem().unwrap().to_string_lossy().to_string();

                let mut frontmatter_str = "";
                let mut main_content_str = full_content.as_str();

                if full_content.starts_with("---") {
                    // Find the end of the frontmatter block
                    // Look for the *second* occurrence of "\n---", which marks the true end of the YAML block
                    // Add 3 to skip the initial "---"
                    if let Some(fm_content_end_offset) = full_content[3..].find("\n---") {
                        let actual_fm_content_end = 3 + fm_content_end_offset;
                        frontmatter_str = &full_content[..actual_fm_content_end]; // The YAML content itself, excluding final delimiter
                        
                        // Content starts after the "\n---" and its following newline
                        let content_start_offset = actual_fm_content_end + "\n---".len();
                        if content_start_offset < full_content.len() {
                            // Skip the newline character that might follow the delimiter
                            if full_content.as_bytes().get(content_start_offset) == Some(&b'\n') {
                                main_content_str = &full_content[content_start_offset + 1..];
                            } else {
                                main_content_str = &full_content[content_start_offset..];
                            }
                        } else {
                            main_content_str = ""; // No content after frontmatter
                        }
                    } else {
                        // Starts with --- but no proper second delimiter found. 
                        // This could be an unterminated frontmatter or just a line of dashes.
                        // Treat as no valid frontmatter in this case.
                        main_content_str = full_content.as_str();
                        frontmatter_str = "";
                        println!("Warning: Document {} starts with --- but no clear end delimiter found. Treating as no frontmatter.", file_name_id);
                    }
                }
                
                let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)
                    .unwrap_or_else(|e| {
                        if !frontmatter_str.is_empty() {
                            println!(
                                "Warning: Failed to parse frontmatter for {}: {}. Frontmatter was: '{}...'. Using defaults.",
                                file_name_id,
                                e,
                                frontmatter_str.chars().take(100).collect::<String>() // Print more for debugging
                            );
                        }
                        Frontmatter { 
                            title: file_name_id.replace("-", " "),
                            tags: None, 
                            rekt: None, 
                            excerpt: None 
                        }
                    });

                let term_freq = self.process_text(
                    main_content_str, 
                    &frontmatter.tags.clone().unwrap_or_default(),
                    &frontmatter.rekt.as_ref().and_then(|r| r.audit.clone()),
                    &frontmatter.excerpt.clone()
                );
                
                self.documents.push(Document {
                    id: file_name_id,
                    title: frontmatter.title,
                    tags: frontmatter.tags.unwrap_or_default(),
                    auditor: frontmatter.rekt.and_then(|r| r.audit),
                    term_freq,
                });
            }
        }

        self.calculate_idf_scores();
        self.build_term_set();
        self.calculate_tfidf_matrix();
        self.calculate_similarity_matrix();

        let duration = start_time.elapsed();
        println!(
            "Loaded and processed {} documents in {:.2?}",
            self.documents.len(),
            duration
        );
        Ok(())
    }
    
    fn clean_text(&self, text: &str) -> String {
        let text = self.url_pattern.replace_all(text, " ");
        let text = self.crypto_address_pattern.replace_all(&text, " ");
        let text = self.twitter_handle_pattern.replace_all(&text, " ");
        let text = self.markdown_pattern.replace_all(&text, " ");
        let text = self.file_path_pattern.replace_all(&text, " ");

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
        result.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    fn is_valid_term(&self, term: &str) -> bool {
        if term.len() <= 1 { // Allow 2-letter words if they are significant after stemming/n-gram
            return false;
        }
        if term.chars().all(|c| c.is_digit(10)) {
            return false;
        }
        let digit_count = term.chars().filter(|c| c.is_digit(10)).count();
        if term.len() > 2 && digit_count * 2 > term.len() { // if more than half are digits for term > 2
            return false;
        }
        true
    }

    fn process_text(
        &self,
        main_content: &str,
        tags: &[String],
        auditor: &Option<String>,
        excerpt: &Option<String>,
    ) -> HashMap<String, usize> {
        let mut text_to_process = String::new();
        if let Some(e) = excerpt {
            text_to_process.push_str(e);
            text_to_process.push(' '); // Separator
        }
        text_to_process.push_str(main_content);

        let cleaned_text = self.clean_text(&text_to_process);
        let lower_text = cleaned_text.to_lowercase();

        let mut words: Vec<String> = lower_text
            .split(|c: char| !c.is_alphanumeric() && c != '-')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_matches('-').to_string()) 
            .filter(|s| !s.is_empty())
            .collect();

        // Add stemmed tags with boosting
        for tag in tags {
            for sub_tag in tag.split(|c: char| !c.is_alphanumeric()) {
                 if !sub_tag.trim().is_empty() {
                    let stemmed_tag = self.stemmer.stem(&sub_tag.to_lowercase()).to_string();
                    if self.is_valid_term(&stemmed_tag) && !self.stop_words.contains(&stemmed_tag) {
                         for _ in 0..TAG_BOOST { words.push(stemmed_tag.clone()); }
                    }
                }
            }
        }

        // Add stemmed auditor name with boosting
        if let Some(aud) = auditor {
            let cleaned_auditor = aud.to_lowercase().replace(" ", "");
            let stemmed_auditor = self.stemmer.stem(&cleaned_auditor).to_string();
            if self.is_valid_term(&stemmed_auditor) && !self.stop_words.contains(&stemmed_auditor) {
                for _ in 0..AUDITOR_BOOST { words.push(stemmed_auditor.clone()); }
            }
        }

        let base_tokens: Vec<String> = words
            .into_iter()
            .filter(|word| self.is_valid_term(word))
            .map(|word| self.stemmer.stem(&word).to_string())
            .filter(|stemmed_word| !self.stop_words.contains(stemmed_word))
            .collect();

        let mut processed_words_with_ngrams = base_tokens.clone();

        // Generate Bi-grams
        if base_tokens.len() >= 2 {
            for i in 0..(base_tokens.len() - 1) {
                let bigram = format!("{}_{}", base_tokens[i], base_tokens[i+1]);
                processed_words_with_ngrams.push(bigram);
            }
        }

        // Generate Tri-grams
        if base_tokens.len() >= 3 {
            for i in 0..(base_tokens.len() - 2) {
                let trigram = format!("{}_{}_{}", base_tokens[i], base_tokens[i+1], base_tokens[i+2]);
                processed_words_with_ngrams.push(trigram);
            }
        }

        let mut term_frequencies: HashMap<String, usize> = HashMap::new();
        for word in processed_words_with_ngrams {
            *term_frequencies.entry(word).or_insert(0) += 1;
        }

        term_frequencies.retain(|_, &mut count| count > 1);
        term_frequencies
    }

    fn calculate_idf_scores(&mut self) {
        let total_docs = self.documents.len() as f32;
        let mut term_doc_count: HashMap<String, usize> = HashMap::new();

        for doc in &self.documents {
            for term in doc.term_freq.keys() {
                *term_doc_count.entry(term.clone()).or_insert(0) += 1;
            }
        }

        let max_doc_threshold = (total_docs * MAX_DOC_PERCENTAGE) as usize;

        for (term, count) in term_doc_count {
            // Distinguish min threshold for unigrams vs n-grams
            let min_threshold = if term.contains('_') { // Simple check for n-gram
                MIN_DOC_THRESHOLD_NGRAM
            } else {
                MIN_DOC_THRESHOLD_UNIGRAM
            };

            if count >= min_threshold && count <= max_doc_threshold {
                let idf = (total_docs / (count as f32 + 1.0)).ln() + 1.0;
                self.idf_scores.insert(term, idf);
            }
        }
    }

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
        self.term_set.sort(); 
    }

    fn calculate_tfidf(&self, term: &str, doc: &Document) -> f32 {
        if !self.idf_scores.contains_key(term) {
            return 0.0;
        }
        let term_count = *doc.term_freq.get(term).unwrap_or(&0);
        if term_count == 0 {
            return 0.0;
        }
        let max_count = doc.term_freq.values().max().unwrap_or(&1);
        let tf = 0.5 + 0.5 * (term_count as f32 / *max_count as f32);
        let idf = self.idf_scores.get(term).unwrap_or(&0.0);
        tf * idf
    }
    
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

    fn calculate_similarity_matrix(&mut self) {
        let start_time = Instant::now();
        let total_docs = self.documents.len();

        let mut graph = Graph::new();
        let tfidf_data: Vec<f32> = self
            .tfidf_matrix
            .iter()
            .flat_map(|row| row.iter().map(|&val| val as f32))
            .collect();

        let total_terms = self.term_set.len();
        if total_terms == 0 { // Guard against empty term set
            println!("Warning: Term set is empty. Cannot calculate similarity matrix.");
            self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];
            return;
        }

        let tfidf_tensor = graph.tensor((total_docs, total_terms)).set(tfidf_data.clone());
        let squared = tfidf_tensor * tfidf_tensor;
        let sum_squared = squared.sum_reduce(1);
        let magnitudes = sum_squared.sqrt() + 1e-1;
        let  normalized_tfidf = tfidf_tensor / magnitudes.expand(1, total_terms);
        let mut similarities_tensor = normalized_tfidf.matmul(normalized_tfidf.permute((1, 0))).retrieve();


        graph.compile(<(
        GenericCompiler, 
        StwoCompiler
        )>::default(), &mut similarities_tensor);

        let time = Instant::now();
        let mut settings = graph.gen_circuit_settings();
        println!("Setting gen in {:?}", time.elapsed());

        let time = Instant::now();
        let pie = graph.gen_trace(&mut settings).expect("Trace generation failed");
        println!("Trace gen in {:?}", time.elapsed());
        println!("Max LogSize {:?}", pie.execution_resources.max_log_size);

        let time = Instant::now();
        let proof = graph.prove(pie, settings.clone()).expect("Proving failed");
        println!("Proof gen in {:?}", time.elapsed());

        let time = Instant::now();
        graph.verify(proof, settings).expect("Verification failed");
        println!("Verified in {:?} ✅", time.elapsed());


        let similarity_data = similarities_tensor.data();

        self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];
        for i in 0..total_docs {
            for j in 0..total_docs {
                self.similarity_matrix[i][j] = similarity_data[i * total_docs + j];
            }
        }
        for i in 0..total_docs {
            self.similarity_matrix[i][i] = 1.0;
        }
        let duration = start_time.elapsed();
        println!(
            "Similarity matrix calculated using Luminal in {:.2?}",
            duration
        );
    }

    fn get_recommendations(&self, doc_id: &str, num_recommendations: usize) -> Vec<(String, f32)> {
        let doc_idx = self.documents.iter().position(|d| d.id == doc_id);
        if let Some(idx) = doc_idx {
            if self.similarity_matrix.is_empty() || idx >= self.similarity_matrix.len() {
                println!("Warning: Similarity matrix not properly initialized or index out of bounds for {}.
", doc_id);
                return Vec::new();
            }
            let mut similarities: Vec<(usize, f32)> = self.similarity_matrix[idx]
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != idx)
                .map(|(i, &score)| (i, score))
                .collect();
            similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            similarities
                .into_iter()
                .take(num_recommendations)
                .map(|(doc_idx_rec, score)| (self.documents[doc_idx_rec].id.clone(), score))
                .collect()
        } else {
            println!("Document with ID '{}' not found", doc_id);
            Vec::new()
        }
    }

    fn print_document_details(&self, doc_id: &str) {
        if let Some(doc) = self.documents.iter().find(|d| d.id == doc_id) {
            println!("\n--- Details for Document: {} (ID: {}) ---", doc.title, doc.id);
            if !doc.tags.is_empty() {
                println!("  Tags: {}", doc.tags.join(", "));
            }
            if let Some(aud) = &doc.auditor {
                println!("  Auditor: {}", aud);
            }

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
    recommender.load_documents(Path::new("rekt_articles"))?;

    let example_article = "wintermute-rekt";
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

