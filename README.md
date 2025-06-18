# REKT Recommender

A zk-based verifiable recommender system for recommending articles from Rekt News. It generates similarity matrices and provides cryptographic proofs to ensure the integrity of the computation. The current system is content-based, with plans underway to evolve into a hybrid recommender by incorporating collaborative filtering techniques.

## Why Does This Matter?
We believe no news site or social platform should control what you see behind the scenes. Our mission is to build a web where integrity and transparency come first.



## 🏗️ Architecture

### System Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   API Layer     │    │  Text Processor │    │  ZK Proof Gen   │
│                 │    │                 │    │                 │
│ • Authentication│────│ • Cleaning      │────│ • LuminAIR      │
│ • Request Route │    │ • Stemming      │    │ • Verification  │
│ • Error Handle  │    │ • N-gram Gen    │    │ • Serialization │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                  ┌─────────────────┐
                  │ Similarity Calc │
                  │                 │
                  │ • TF-IDF Matrix │
                  │ • Cosine Sim    │
                  │ • Result Export │
                  └─────────────────┘
```

### Processing Pipeline

1. **Input Validation**: JSON structure validation and content type verification
2. **Text Preprocessing**: 
   - URL and crypto address removal
   - Markdown syntax cleaning
   - Stop word filtering
   - Word stemming and normalization
3. **Feature Extraction**:
   - Term frequency calculation
   - N-gram generation (bigrams and trigrams)
   - Tag and auditor boosting
4. **TF-IDF Computation**:
   - Inverse document frequency calculation
   - Term filtering by document frequency
   - Matrix construction
5. **Similarity Analysis** (Verifiable):
   - Vector normalization
   - Cosine similarity computation
   - Zero-knowledge proof generation
6. **Result Storage**:
   - Matrix serialization
   - Proof data export
   - Metadata generation

## 📡 API Endpoints

### Health Check
```http
GET /health
```

**Response:**
```json
{
  "status": "healthy",
  "service": "rekt-recommender-api",
  "version": "0.1.3"
}
```

### Process Articles
```http
POST /process
Content-Type: application/json
X-API-Key: your-api-key
```

**Request Body:**
```json
{
  "timestamp": 1640995200,
  "posts": [
    {
      "date": "12/31/2023",
      "title": "DeFi Protocol Exploit Analysis",
      "excerpt": "Detailed analysis of the recent exploit...",
      "slug": "defi-protocol-exploit-analysis",
      "tags": ["DeFi", "Security", "Analysis"],
      "rekt": {
        "amount": 50000000,
        "audit": "Trail of Bits",
        "date": "12/30/2023"
      }
    }
  ]
}
```

**Response:**
```json
{
  "request_id": "123e4567-e89b-12d3-a456-426614174000",
  "status": "success",
  "message": "Articles processed successfully",
  "data": {
    "articles_processed": 45,
    "similarity_matrix_shape": [45, 45],
    "proof_size_bytes": 2048,
    "output_directory": "./outputs/result_123e4567-e89b-12d3-a456-426614174000"
  }
}
```

### Download Results
```http
GET /download/{request_id}
X-API-Key: your-api-key
```

**Response:** ZIP file containing:
- `article_ids.json` - List of processed article identifiers
- `similarity_matrix.json` - Computed similarity matrix
- `proof.bin` - Zero-knowledge proof data
- `circuit_settings.bin` - Proof verification settings
- `metadata.json` - Processing metadata and statistics

### Manual Cleanup
```http
POST /cleanup
X-API-Key: your-api-key
```

**Response:**
```json
{
  "request_id": "123e4567-e89b-12d3-a456-426614174000",
  "status": "success",
  "message": "Manual cleanup completed. Removed directories older than 24 hours."
}
```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `PORT` | Server port | `8080` | No |
| `MAX_UPLOAD_SIZE_MB` | Maximum upload size in MB | `100` | No |
| `API_KEY` | Authentication key (min 16 chars) | - | **Yes** |
| `OUTPUT_DIR` | Results storage directory | `./outputs` | No |
| `RESULT_TTL_HOURS` | Result retention time in hours | `24` | No |

### Text Processing Configuration

The system uses configurable parameters for text processing:

- **Tag Boost**: `5x` - Multiplier for tag term importance
- **Auditor Boost**: `3x` - Multiplier for auditor term importance
- **Max Document Percentage**: `80%` - Maximum document frequency for terms
- **Min Document Thresholds**: 
  - Unigrams: `5` documents minimum
  - N-grams: `2` documents minimum

## 🚀 Quick Start

### Prerequisites
- Rust 1.70+ with Cargo
- Git

### Installation

1. **Clone the repository:**
   ```bash
   git clone <repository-url>
   cd rekt-recommender-api
   ```

2. **Set up environment variables:**
   ```bash
   export API_KEY="your-secure-api-key-here-min-16-chars"
   export PORT=8080
   export OUTPUT_DIR="./outputs"
   export RESULT_TTL_HOURS=24
   ```

3. **Build and run:**
   ```bash
   # Development
   cargo run
   
   # Production
   cargo build --release
   ./target/release/rekt-recommender-api
   ```

### Docker Deployment

1. **Build the image:**
   ```bash
   docker build -t rekt-recommender .
   ```

2. **Run the container:**
   ```bash
   docker run -d \
     -p 8080:8080 \
     -e API_KEY="your-secure-api-key" \
     -e OUTPUT_DIR="/app/outputs" \
     -v $(pwd)/outputs:/app/outputs \
     rekt-recommender
   ```

### Google Cloud Build

The project includes a `cloudbuild.yaml` configuration for automated deployment to Google Cloud:

```bash
gcloud builds submit --config cloudbuild.yaml .
```

## 🧪 Usage Examples

### Basic Article Processing

```bash
# Process articles from JSON file
curl -X POST http://localhost:8080/process \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d @articles.json
```

### Download Results

```bash
# Download processing results
curl -X GET http://localhost:8080/download/123e4567-e89b-12d3-a456-426614174000 \
  -H "X-API-Key: your-api-key" \
  -o results.zip
```

### Health Check

```bash
# Check service health
curl http://localhost:8080/health
```

## 🏗️ Development

### Project Structure

```
src/
├── main.rs                 # Application entry point and server setup
├── api.rs                  # HTTP handlers and authentication
├── config.rs               # Configuration management
├── types.rs                # Data structures and serialization
├── utils.rs                # Utility functions for file operations
└── recommender/
    ├── mod.rs              # Main recommender system logic
    └── text_processing.rs  # Natural language processing
```

### Key Components

- **RecommenderSystem**: Main orchestrator for the processing pipeline
- **TextProcessor**: Handles all text cleaning and feature extraction
- **ApiKey**: Request guard for authentication
- **CleanupFairing**: Background task manager for result cleanup

### Building from Source

```bash
# Install dependencies
cargo build

# Run tests
cargo test

# Check for linting issues
cargo clippy

# Format code
cargo fmt
```

## 🔗 Links

- [LuminAIR zkML Framework](https://github.com/gizatechxyz/LuminAIR)
- [REKT News](https://rekt.news/)

---