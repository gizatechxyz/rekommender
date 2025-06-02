# REKT Recommender API

A Cloud Run service that processes markdown articles to calculate TF-IDF scores, generate cosine similarity matrices, and create cryptographic proofs using Luminal.

## Features

- Processes zip files containing markdown articles
- Calculates TF-IDF scores with n-gram support
- Generates cosine similarity matrices
- Creates cryptographic proofs using Luminal/Luminair
- Returns results as a zip file containing:
  - Article IDs (JSON)
  - Similarity matrix (JSON)
  - Cryptographic proof (binary)
  - Processing metadata (JSON)

## API Endpoints

### Health Check
```
GET /health
```

Returns service health status.

### Process Articles
```
POST /process
Content-Type: application/zip
Body: ZIP file containing markdown articles
```

Returns a ZIP file with processing results.

## Input Format

The input should be a ZIP file containing markdown articles with YAML frontmatter:

```markdown
---
title: Article Title
date: 12/08/2021
rekt:
  amount: 1750000
  audit: Unaudited
  date: 12/08/2021
tags:
  - tag1
  - tag2
excerpt: Brief description
---

Article content here...
```

## Output Format

The API returns a ZIP file containing:

1. **article_ids.json** - Array of processed article IDs
2. **similarity_matrix.json** - 2D array of similarity scores
3. **proof.bin** - Binary cryptographic proof
4. **metadata.json** - Processing metadata including:
   - Request ID
   - Timestamp
   - Number of articles processed
   - Matrix dimensions
   - Proof size

## Deployment

### Prerequisites

- Google Cloud Project with Cloud Run API enabled
- Docker installed locally (for local testing)
- gcloud CLI configured

### Deploy to Cloud Run

1. Build and deploy using Cloud Build:
```bash
gcloud builds submit --config cloudbuild.yaml
```

2. Or deploy manually:
```bash
# Build image
docker build -t gcr.io/YOUR_PROJECT_ID/rekt-recommender-api .

# Push to Container Registry
docker push gcr.io/YOUR_PROJECT_ID/rekt-recommender-api

# Deploy to Cloud Run
gcloud run deploy rekt-recommender-api \
  --image gcr.io/YOUR_PROJECT_ID/rekt-recommender-api \
  --region us-central1 \
  --platform managed \
  --memory 2Gi \
  --cpu 2 \
  --timeout 300 \
  --max-instances 10 \
  --set-env-vars MAX_UPLOAD_SIZE_MB=100
```

### Environment Variables

- `PORT` - Server port (default: 8080)
- `MAX_UPLOAD_SIZE_MB` - Maximum upload size in MB (default: 100)

## Local Development

1. Install Rust and dependencies:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Build and run:
```bash
cargo build --release
cargo run --release
```

3. Test with curl:
```bash
# Create a test zip file
cd rekt_articles && zip -r ../rekt_articles.zip *.md && cd ..

# Send request
curl -X POST http://localhost:8080/process \
  -H "Content-Type: application/zip" \
  --data-binary @rekt_articles.zip \
  --output results.zip
```

## Authorization

To add authorization to your Cloud Run service:

1. Deploy with `--no-allow-unauthenticated` flag:
```bash
gcloud run deploy rekt-recommender-api \
  --image gcr.io/YOUR_PROJECT_ID/rekt-recommender-api \
  --no-allow-unauthenticated \
  --region us-central1
```

2. Grant access to specific service accounts:
```bash
gcloud run services add-iam-policy-binding rekt-recommender-api \
  --member="serviceAccount:YOUR_SERVICE_ACCOUNT@YOUR_PROJECT.iam.gserviceaccount.com" \
  --role="roles/run.invoker" \
  --region us-central1
```

3. Call the API with authentication:
```bash
# Get auth token
TOKEN=$(gcloud auth print-identity-token)

# Make authenticated request
curl -X POST https://YOUR_CLOUD_RUN_URL/process \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/zip" \
  --data-binary @test_articles.zip \
  --output results.zip
```

## Performance Considerations

- The service processes articles in memory
- Large document sets may require increased memory allocation
- Proof generation is computationally intensive
- Consider using Cloud Run's CPU boost feature for better cold start performance

## Error Handling

The API returns JSON error responses with the following structure:
```json
{
  "request_id": "uuid",
  "status": "error",
  "message": "Error description",
  "data": null
}
```

Common error codes:
- 400: Invalid request (wrong content type, invalid zip)
- 413: Payload too large
- 500: Internal server error

## Monitoring

Monitor your service using:
- Cloud Run metrics in Google Cloud Console
- Cloud Logging for application logs
- Cloud Trace for performance analysis

## License

[Your License Here]