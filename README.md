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