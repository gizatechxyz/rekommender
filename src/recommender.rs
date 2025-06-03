use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result};
use luminair::prelude::*;
use regex::Regex;
use rust_stemmers::{Algorithm, Stemmer};
use serde::Deserialize;
use stop_words;

// Recommender weights
const TAG_BOOST: usize = 5;
const AUDITOR_BOOST: usize = 3;
const MAX_DOC_PERCENTAGE: f32 = 0.80;
const MAX_FILES: usize = 70;
const MIN_DOC_THRESHOLD_UNIGRAM: usize = 5;
const MIN_DOC_THRESHOLD_NGRAM: usize = 2;

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
    id: String,
    title: String,
    tags: Vec<String>,
    auditor: Option<String>,
    term_freq: HashMap<String, usize>,
}

pub struct RecommenderSystem {
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
    proof_data: Vec<u8>,
    circuit_settings: Vec<u8>,
}

impl RecommenderSystem {
    pub fn new() -> Self {
        let stemmer = Stemmer::create(Algorithm::English);
        let mut stop_words_vec = stop_words::get(stop_words::LANGUAGE::English);

        let rekt_stop_words_originals = vec![
            "eth",
            "btc",
            "defi",
            "crypto",
            "rekt",
            "blockchain",
            "token",
            "tokens",
            "https",
            "http",
            "com",
            "www",
            "twitter",
            "tweet",
            "status",
            "attack",
            "exploit",
        ];

        stop_words_vec.extend(rekt_stop_words_originals.iter().map(|s| s.to_string()));

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
            proof_data: Vec::new(),
            circuit_settings: Vec::new(),
        }
    }

    pub fn load_and_process(
        &mut self,
        directory: &Path,
    ) -> Result<(Vec<String>, Vec<u8>, Vec<u8>)> {
        let start_time = Instant::now();

        // Load documents
        self.load_documents(directory)?;

        // Calculate everything
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

    fn load_documents(&mut self, directory: &Path) -> Result<()> {
        let mut file_entries = Vec::new();

        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                let metadata = fs::metadata(&path)?;
                let modified_time = metadata
                    .modified()
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
                file_entries.push((path, modified_time));
            }
        }

        file_entries.sort_by(|a, b| b.1.cmp(&a.1));

        // Keep only the first
        if file_entries.len() > MAX_FILES {
            file_entries.truncate(MAX_FILES);
        }

        for (path, _) in file_entries {
            let full_content = fs::read_to_string(&path)?;
            let file_name_id = path.file_stem().unwrap().to_string_lossy().to_string();

            let mut frontmatter_str = "";
            let mut main_content_str = full_content.as_str();

            if full_content.starts_with("---") {
                if let Some(fm_content_end_offset) = full_content[3..].find("\n---") {
                    let actual_fm_content_end = 3 + fm_content_end_offset;
                    frontmatter_str = &full_content[..actual_fm_content_end];

                    let content_start_offset = actual_fm_content_end + "\n---".len();
                    if content_start_offset < full_content.len() {
                        if full_content.as_bytes().get(content_start_offset) == Some(&b'\n') {
                            main_content_str = &full_content[content_start_offset + 1..];
                        } else {
                            main_content_str = &full_content[content_start_offset..];
                        }
                    } else {
                        main_content_str = "";
                    }
                }
            }

            let frontmatter: Frontmatter =
                serde_yaml::from_str(frontmatter_str).unwrap_or_else(|_| Frontmatter {
                    title: file_name_id.replace("-", " "),
                    tags: None,
                    rekt: None,
                    excerpt: None,
                });

            let term_freq = self.process_text(
                main_content_str,
                &frontmatter.tags.clone().unwrap_or_default(),
                &frontmatter.rekt.as_ref().and_then(|r| r.audit.clone()),
                &frontmatter.excerpt.clone(),
            );

            self.documents.push(Document {
                id: file_name_id,
                title: frontmatter.title,
                tags: frontmatter.tags.unwrap_or_default(),
                auditor: frontmatter.rekt.and_then(|r| r.audit),
                term_freq,
            });
        }

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
        if term.len() <= 1 {
            return false;
        }
        if term.chars().all(|c| c.is_digit(10)) {
            return false;
        }
        let digit_count = term.chars().filter(|c| c.is_digit(10)).count();
        if term.len() > 2 && digit_count * 2 > term.len() {
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
            text_to_process.push(' ');
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

        for tag in tags {
            for sub_tag in tag.split(|c: char| !c.is_alphanumeric()) {
                if !sub_tag.trim().is_empty() {
                    let stemmed_tag = self.stemmer.stem(&sub_tag.to_lowercase()).to_string();
                    if self.is_valid_term(&stemmed_tag) && !self.stop_words.contains(&stemmed_tag) {
                        for _ in 0..TAG_BOOST {
                            words.push(stemmed_tag.clone());
                        }
                    }
                }
            }
        }

        if let Some(aud) = auditor {
            let cleaned_auditor = aud.to_lowercase().replace(" ", "");
            let stemmed_auditor = self.stemmer.stem(&cleaned_auditor).to_string();
            if self.is_valid_term(&stemmed_auditor) && !self.stop_words.contains(&stemmed_auditor) {
                for _ in 0..AUDITOR_BOOST {
                    words.push(stemmed_auditor.clone());
                }
            }
        }

        let base_tokens: Vec<String> = words
            .into_iter()
            .filter(|word| self.is_valid_term(word))
            .map(|word| self.stemmer.stem(&word).to_string())
            .filter(|stemmed_word| !self.stop_words.contains(stemmed_word))
            .collect();

        let mut processed_words_with_ngrams = base_tokens.clone();

        if base_tokens.len() >= 2 {
            for i in 0..(base_tokens.len() - 1) {
                let bigram = format!("{}_{}", base_tokens[i], base_tokens[i + 1]);
                processed_words_with_ngrams.push(bigram);
            }
        }

        if base_tokens.len() >= 3 {
            for i in 0..(base_tokens.len() - 2) {
                let trigram = format!(
                    "{}_{}_{}",
                    base_tokens[i],
                    base_tokens[i + 1],
                    base_tokens[i + 2]
                );
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
            let min_threshold = if term.contains('_') {
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

    fn calculate_similarity_matrix_with_proof(&mut self) -> Result<()> {
        let start_time = Instant::now();
        let total_docs = self.documents.len();

        if self.term_set.is_empty() {
            self.similarity_matrix = vec![vec![0.0; total_docs]; total_docs];
            return Ok(());
        }

        let mut graph = Graph::new();
        let tfidf_data: Vec<f32> = self
            .tfidf_matrix
            .iter()
            .flat_map(|row| row.iter().map(|&val| val as f32))
            .collect();

        let total_terms = self.term_set.len();
        let tfidf_tensor = graph
            .tensor((total_docs, total_terms))
            .set(tfidf_data.clone());
        let squared = tfidf_tensor * tfidf_tensor;
        let sum_squared = squared.sum_reduce(1);
        let magnitudes = sum_squared.sqrt() + 1e-1;
        let normalized_tfidf = tfidf_tensor / magnitudes.expand(1, total_terms);
        let mut similarities_tensor = normalized_tfidf
            .matmul(normalized_tfidf.permute((1, 0)))
            .retrieve();

        graph.compile(
            <(GenericCompiler, StwoCompiler)>::default(),
            &mut similarities_tensor,
        );

        let mut circuit_settings = graph.gen_circuit_settings();
        let pie = graph
            .gen_trace(&mut circuit_settings)
            .context("Trace generation failed")?;
        let proof = prove(pie, circuit_settings.clone()).context("Proving failed")?;

        // Serialize proof and circuit_settings to binary
        self.proof_data = proof.to_bincode().context("Failed to serialize proof")?;
        self.circuit_settings = circuit_settings
            .to_bincode()
            .context("Failed to serialize circuit_settings")?;

        verify(proof, circuit_settings).context("Verification failed")?;

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
            "Similarity matrix calculated and proof generated in {:.2?}",
            duration
        );

        Ok(())
    }

    pub fn get_similarity_matrix(&self) -> Vec<Vec<f32>> {
        self.similarity_matrix.clone()
    }
}
