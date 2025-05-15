# Rekt Rekommender

This project implements a verifiable content-based recommender system, specifically designed to recommend similar articles from a collection of "rekt_articles".
The system analyzes the textual content of these articles, including their YAML frontmatter, to identify and rank similar incidents and generates a ZK proof of the similarity calcul.

## Core Strategy & Methodology

The recommender system employs the following strategy to identify and recommend similar articles:

1.  **Document Loading and Parsing:**

    - Reads all markdown (`.md`) files from the `rekt_articles/` directory.
    - Parses YAML frontmatter from each article to extract:
      - `title`: The official title of the article.
      - `tags`: Relevant keywords or categories.
      - `auditor`: The name of the auditing firm, if mentioned in the `rekt.audit` field.
      - `excerpt`: A summary or introductory part of the article.
    - The filename (without the `.md` extension) serves as the unique `id` for each document.

2.  **Text Preprocessing and Feature Engineering:**

    - **Concatenation**: The `excerpt` (if present) is prepended to the main markdown content for a consolidated text body.
    - **Cleaning**:
      - Removes URLs, cryptocurrency addresses (0x format and others), Twitter handles, common file extensions, and markdown syntax (like image tags, emphasis, headers) using regular expressions.
      - Replaces various punctuation marks with spaces to ensure proper tokenization.
    - **Tokenization**: Splits the cleaned text into words (tokens) based on non-alphanumeric characters. Hyphens within words are preserved initially.
    - **Normalization**: Converts all tokens to lowercase.
    - **Frontmatter Feature Boosting**:
      - **Tags**: Tags from the frontmatter are split, stemmed, and added multiple times (controlled by `TAG_BOOST`) to the document's word list. This increases their weight in similarity calculations.
      - **Auditor**: The auditor's name (if present) is cleaned, stemmed, and added multiple times (controlled by `AUDITOR_BOOST`).
    - **Stemming**: Reduces words to their root form using the English Snowball stemmer (`rust-stemmers` crate). This helps group related words (e.g., "attack", "attacking", "attacked" all become "attack").
    - **Stop Word Removal**:
      - Removes common English stop words (`stop-words` crate).
      - Removes a custom list of domain-specific stop words (e.g., "eth", "btc", "rekt", "crypto", "https", "twitter", "attack", "exploit").
      - Importantly, stemming is performed _before_ stop word removal, and the custom stop word list includes both original and stemmed versions to ensure comprehensive filtering.
    - **N-gram Generation**:
      - Generates bi-grams (sequences of two adjacent words) and tri-grams (sequences of three adjacent words) from the stemmed, stop-word-filtered token list. These n-grams are treated as single terms (e.g., "smart_contract", "flash_loan_attack").
    - **Term Validation and Filtering**:
      - Filters out very short terms (1 character).
      - Filters out purely numeric terms or terms that are mostly numeric.
      - Retains only terms (unigrams and n-grams) that appear more than once within a single document to reduce noise from incidental words.

3.  **TF-IDF (Term Frequency-Inverse Document Frequency) Calculation:**

    - **IDF (Inverse Document Frequency) Calculation**:
      - Calculates IDF scores for all unique terms across the entire corpus.
      - Uses a smoothed IDF formula: `ln(total_docs / (doc_frequency + 1.0)) + 1.0`.
      - Filters terms based on document frequency:
        - Unigrams must appear in at least `MIN_DOC_THRESHOLD_UNIGRAM` documents.
        - N-grams must appear in at least `MIN_DOC_THRESHOLD_NGRAM` documents.
        - Terms appearing in more than `MAX_DOC_PERCENTAGE` of documents are excluded (as they are too common to be discriminative).
    - **Term Set Creation**: Builds a sorted list of all valid terms (those with IDF scores) found in the corpus.
    - **TF (Term Frequency) Calculation**:
      - For each document, calculates the frequency of each term from the global `term_set`.
      - Uses augmented term frequency to prevent bias towards longer documents: `0.5 + 0.5 * (term_count / max_term_count_in_doc)`.
    - **TF-IDF Matrix Construction**: Creates a matrix where rows represent documents and columns represent terms from the `term_set`. Each cell `(doc, term)` contains the TF-IDF score of that term in that document.

4.  **Verifiable Similarity Calculation (Cosine Similarity with LuminAIR):**

    - The core of the similarity calculation is performed using the `LuminAIR` crate for efficient tensor operations, mimicking `sklearn.metrics.pairwise.linear_kernel` on normalized vectors.
    - **TF-IDF Matrix to Tensor**: The `tfidf_matrix` (`Vec<Vec<f32>>`) is converted into a flat `Vec<f32>` and then into a `LuminAIR` tensor.
    - **L2 Normalization**: Each document's TF-IDF vector (row in the tensor) is L2 normalized. This is done by:
      1.  Squaring each element in the TF-IDF tensor.
      2.  Summing the squared elements along each row (axis 1) to get the squared magnitude of each document vector.
      3.  Taking the square root of these sums to get the magnitudes.
      4.  A small epsilon (1e-8) is added to magnitudes before division to prevent division by zero.
      5.  Dividing the original TF-IDF tensor by these expanded magnitudes.
    - **Cosine Similarity via Matrix Multiplication**: The cosine similarity between all pairs of documents is calculated by performing a matrix multiplication of the normalized TF-IDF tensor with its transpose (`normalized_tfidf.matmul(normalized_tfidf.transpose())`).
    - The resulting similarity scores are retrieved from the `LuminAIR` graph and stored in a `similarity_matrix` (Vec<Vec<f32>>). Self-similarity (diagonal elements) is explicitly set to 1.0.
    - This entire similarity matrix is pre-computed when documents are loaded.

5.  **Generating Recommendations:**
    - Given a target document ID, the system looks up its corresponding row in the pre-computed `similarity_matrix`.
    - The scores in this row represent the similarity of the target document to all other documents.
    - These scores are sorted in descending order.
    - The top `N` documents (excluding the target document itself) are returned as recommendations, along with their similarity scores.

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
