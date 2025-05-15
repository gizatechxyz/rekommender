# Rekt Rekommender

This project implements a verifiable content-based recommender system, specifically designed to recommend similar articles from a collection of "rekt_articles".
The system analyzes the textual content of these articles, including their YAML frontmatter, to identify and rank similar incidents and generates a ZK proof of the similarity calcul.

🏗️ Note that this project is in active development. 

## Core Strategy & Methodology

The recommender system identifies similar articles through these core steps:

1.  **Document Loading & Parsing:**
    *   Loads markdown (`.md`) files from `rekt_articles/`.
    *   Extracts `title`, `tags`, `auditor`, and `excerpt` from YAML frontmatter. Article filenames serve as unique IDs.

2.  **Text Preprocessing & Feature Engineering:**
    *   Combines `excerpt` with main content.
    *   Cleans text by removing URLs, crypto addresses, Twitter handles, markdown syntax, and standardizing punctuation.
    *   Tokenizes text, normalizes to lowercase, and applies stemming (reducing words to roots).
    *   Boosts terms from `tags` and `auditor` names by adding them multiple times to the document's word list.
    *   Removes common English and domain-specific stop words (stemming occurs *before* stop word removal for accuracy).
    *   Generates bi-grams and tri-grams (e.g., "smart_contract") from processed tokens.
    *   Filters terms: removes very short, numeric/mostly-numeric terms, and terms appearing only once per document.

3.  **TF-IDF Calculation:**
    *   Calculates smoothed Inverse Document Frequency (IDF) scores for all unique terms, filtering by minimum and maximum document frequency thresholds (different for unigrams and n-grams).
    *   Constructs a TF-IDF matrix: documents (rows) vs. terms (columns), with each cell containing the TF-IDF score (using augmented term frequency).

4.  **Verifiable Similarity Calculation (Cosine Similarity with LuminAIR):**
    *   Uses `LuminAIR` for efficient tensor-based cosine similarity.
    *   The TF-IDF matrix is converted to a `LuminAIR` tensor.
    *   Each document's TF-IDF vector is L2 normalized.
    *   The cosine similarity matrix is computed by multiplying the normalized TF-IDF tensor by its transpose. This matrix is pre-computed on load.

5.  **Generating Recommendations:**
    *   For a given article, its similarity scores with all other articles are retrieved from the pre-computed similarity matrix.
    *   Scores are sorted, and the top N articles are returned as recommendations.

## How to Run

1.  **Prerequisites**: Ensure you have Rust installed.
2.  **Clone the Repository**
3.  **Place Articles**: Add your markdown articles into the `rekt_articles/` directory.
4.  **Run the Application**:
    ```bash
    cargo run --release
    ```
    The `--release` flag is recommended for better performance.

The `main` function in `src/main.rs` demonstrates an example usage:

- It initializes the `RecommenderSystem`.
- Loads and processes documents from the `rekt_articles` directory.
- Prints details for an example article (`wintermute-rekt`).
- Fetches and prints the top 5 recommendations for that article, including their similarity scores and details.
