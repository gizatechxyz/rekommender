# Rekt News Verifiable Recommender System

A cryptographically verifiable article recommendation system for Rekt News using STARK proofs. This system ensures that article recommendations are computed transparently and can be verified by users.

## 🔗 System Overview

The Rekt News Verifiable Recommender System provides cryptographic proof that article recommendations are computed using cosine similarity on the actual article content, preventing manipulation or bias in the recommendation algorithm.

### Key Features

- **Verifiable Recommendations**: All recommendations come with STARK proofs
- **Transparent Algorithm**: Uses cosine similarity on article embeddings
- **Zero-Knowledge**: Users can verify recommendations without seeing the underlying computation
- **Frontend Integration**: Easy-to-integrate TypeScript functions and React components

## 📋 System Flow

### 1. Article Processing & Proof Generation

When the Rekt team adds new articles, they upload the updated article dataset (as JSON) to generate fresh similarity calculations and proofs:

```bash
curl -X POST \
  -H "Content-Type: application/json" \
  -H "X-API-Key: 3e6ededaaa438fa4cca129fc674cdde5e84d9275d6dd6068867ca06e0e732d1e" \
  --data-binary @content.json \
  "https://rekt-recommender-api-132737210721.europe-west1.run.app/process"
```

**What happens:**
- The API processes the 70 most recent articles from the JSON input
- Computes embeddings and calculates cosine similarity matrix between all articles
- Generates a STARK proof of the similarity computation
- Returns a JSON response containing the output directory path with:
  - `similarity_matrix.json` - The computed similarities
  - `proof.bin` - STARK proof file
  - `circuit_settings.bin` - Verification settings
  - `metadata.json` - Article metadata and mapping

**Input Format:**
The API now expects a JSON file in the following format:

```json
{
  "timestamp": 1749734388946,
  "posts": [
    {
      "date": "6/9/2025",
      "featured": true,
      "title": "AlexLab - Rekt II",
      "rekt": {
        "amount": 16180000,
        "audit": "Clarity Alliance, CoinFabrik",
        "date": "6/6/2025"
      },
      "tags": ["AlexLab", "Rekt", "Defi"],
      "excerpt": "Over $16 million drained...",
      "banner": "https://...",
      "slug": "alexlab-rekt2"
    }
  ]
}
```

### 2. Frontend Integration

The Rekt team integrates the recommendation system into their website using the provided TypeScript functions.

### 3. User Verification

Readers can verify that recommendations are legitimate using the embedded verification component.

## 🔧 Implementation

### TypeScript Recommendation Function

Add this TypeScript implementation to your frontend codebase:

```typescript
// types/recommender.ts
export interface Article {
  id: string;
  title: string;
  url: string;
  publishDate: string;
  tags?: string[];
  excerpt?: string;
}

export interface SimilarityData {
  similarities: Record<string, Record<string, number>>;
  articles: Record<string, Article>;
  metadata: {
    lastUpdated: string;
    totalArticles: number;
    proofGenerated: string;
  };
}

export interface Recommendation {
  article: Article;
  similarity: number;
  rank: number;
}

// utils/recommender.ts
import { Article, SimilarityData, Recommendation } from '../types/recommender';

/**
 * Fetches the latest similarity data from your storage
 */
async function loadSimilarityData(): Promise<SimilarityData> {
  try {
    // Replace with the actual storage URL or path
    const response = await fetch('/data/similarity_matrix.json');
    if (!response.ok) {
      throw new Error(`Failed to load similarity data: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('Error loading similarity data:', error);
    throw new Error('Failed to load recommendation data');
  }
}

/**
 * Gets top 3 article recommendations based on cosine similarity
 * @param currentArticleId - The ID of the current article being read
 * @param excludeIds - Optional array of article IDs to exclude from recommendations
 * @returns Promise<Recommendation[]> - Array of 3 recommended articles
 */
export async function getRecommendations(
  currentArticleId: string,
  excludeIds: string[] = []
): Promise<Recommendation[]> {
  try {
    const data = await loadSimilarityData();
    
    // Get similarities for the current article
    const currentArticleSimilarities = data.similarities[currentArticleId];
    
    if (!currentArticleSimilarities) {
      console.warn(`No similarities found for article ${currentArticleId}`);
      return [];
    }

    // Convert to array of [articleId, similarity] and sort by similarity
    const sortedSimilarities = Object.entries(currentArticleSimilarities)
      .filter(([articleId]) => {
        // Exclude the current article and any specified exclusions
        return articleId !== currentArticleId && !excludeIds.includes(articleId);
      })
      .filter(([articleId]) => {
        // Ensure the article exists in our metadata
        return data.articles[articleId];
      })
      .sort(([, a], [, b]) => b - a) // Sort by similarity (descending)
      .slice(0, 3); // Take top 3

    // Map to recommendation objects
    const recommendations: Recommendation[] = sortedSimilarities.map(
      ([articleId, similarity], index) => ({
        article: data.articles[articleId],
        similarity,
        rank: index + 1,
      })
    );

    return recommendations;
  } catch (error) {
    console.error('Error getting recommendations:', error);
    return [];
  }
}
```

### React Component Integration

Install the LuminAIR React package and integrate the verification component:

```bash
npm install @gizatech/luminair-react
```

```tsx
// components/ArticleRecommendations.tsx
import React, { useEffect, useState } from 'react';
import { VerifyButton } from '@gizatech/luminair-react';
import '@gizatech/luminair-react/styles.css';
import { getRecommendations, Recommendation } from '../utils/recommender';

interface ArticleRecommendationsProps {
  currentArticleId: string;
  className?: string;
}

export function ArticleRecommendations({ 
  currentArticleId, 
  className = '' 
}: ArticleRecommendationsProps) {
  const [recommendations, setRecommendations] = useState<Recommendation[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadRecommendations() {
      try {
        setLoading(true);
        setError(null);
        const recs = await getRecommendations(currentArticleId);
        setRecommendations(recs);
      } catch (err) {
        setError('Failed to load recommendations');
        console.error('Error loading recommendations:', err);
      } finally {
        setLoading(false);
      }
    }

    loadRecommendations();
  }, [currentArticleId]);

  if (loading) {
    return (
      <div className={`${className} animate-pulse`}>
        <div className="h-4 bg-gray-200 rounded w-1/3 mb-4"></div>
        <div className="space-y-3">
          {[1, 2, 3].map(i => (
            <div key={i} className="h-12 bg-gray-200 rounded"></div>
          ))}
        </div>
      </div>
    );
  }

  if (error || recommendations.length === 0) {
    return (
      <div className={className}>
        <p className="text-gray-500">
          {error || 'No recommendations available at this time.'}
        </p>
      </div>
    );
  }

  return (
    <div className={className}>
      <div className="flex items-center justify-between mb-6">
        <h3 className="text-xl font-bold text-gray-900">
          Recommended Reading
        </h3>
        <VerifyButton
          proofPath="/data/proof.bin"
          settingsPath="/data/settings.bin"
          title="Verify Recommendations"
          buttonText="🔒 Verify"
          author="Rekt News"
          modelDescription="Cosine Similarity Recommender"
          authorUrl="https://rekt.news"
          className="text-sm bg-green-600 hover:bg-green-700 text-white px-3 py-1 rounded"
        />
      </div>

      <div className="space-y-4">
        {recommendations.map((rec) => (
          <article 
            key={rec.article.id}
            className="border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
          >
            <div className="flex items-start justify-between">
              <div className="flex-1">
                <h4 className="font-semibold text-gray-900 hover:text-red-600 transition-colors">
                  <a href={rec.article.url} className="text-decoration-none">
                    {rec.article.title}
                  </a>
                </h4>
                {rec.article.excerpt && (
                  <p className="text-gray-600 text-sm mt-1 line-clamp-2">
                    {rec.article.excerpt}
                  </p>
                )}
                <div className="flex items-center mt-2 text-xs text-gray-500">
                  <span>{new Date(rec.article.publishDate).toLocaleDateString()}</span>
                  {rec.article.tags && rec.article.tags.length > 0 && (
                    <>
                      <span className="mx-2">•</span>
                      <span>{rec.article.tags.slice(0, 2).join(', ')}</span>
                    </>
                  )}
                </div>
              </div>
              <div className="ml-4 text-right">
                <div className="text-xs font-mono text-gray-400">
                  #{rec.rank}
                </div>
                <div className="text-xs text-gray-500">
                  {(rec.similarity * 100).toFixed(1)}% similar
                </div>
              </div>
            </div>
          </article>
        ))}
      </div>

      <div className="mt-4 text-xs text-gray-400 text-center">
        Recommendations are cryptographically verifiable and computed using cosine similarity
      </div>
    </div>
  );
}
```

### Usage in Article Pages

```tsx
// pages/article/[slug].tsx or components/ArticlePage.tsx
import { ArticleRecommendations } from '../components/ArticleRecommendations';

export function ArticlePage({ article }: { article: Article }) {
  return (
    <div className="max-w-4xl mx-auto px-4 py-8">
      {/* Your article content */}
      <article>
        <h1>{article.title}</h1>
        <div dangerouslySetInnerHTML={{ __html: article.content }} />
      </article>

      {/* Recommendations section */}
      <div className="mt-12 border-t pt-8">
        <ArticleRecommendations 
          currentArticleId={article.id}
          className="max-w-2xl"
        />
      </div>
    </div>
  );
}
```

## 🚀 Deployment Setup

### 1. Configure Build System

For **Next.js** projects, update your `next.config.js`:

```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  webpack: (config, { isServer }) => {
    // Handle WASM files for proof verification
    config.experiments = {
      ...config.experiments,
      asyncWebAssembly: true,
    };

    config.module.rules.push({
      test: /\.wasm$/,
      type: 'webassembly/async',
    });

    // Fallback for Node.js modules
    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        path: false,
        crypto: false,
      };
    }

    return config;
  },
};

module.exports = nextConfig;
```

### 2. Setup File Storage

Store the generated files from the API in your public directory or CDN:

```
public/
├── data/
│   ├── similarity_matrix.json
│   ├── proof.bin
│   ├── settings.bin
│   └── metadata.json
```

### 3. Update Workflow

1. **When adding new articles:**
   ```bash
   # Create zip with all articles
   zip -r rekt_articles.zip articles/
   
   # Upload to API
   curl -X POST \
     -H "Content-Type: application/zip" \
     -H "X-API-Key: YOUR_API_KEY" \
     --data-binary @rekt_articles.zip \
     "https://rekt-recommender-api-132737210721.europe-west1.run.app/process" \
     --output result.zip
   
   # Extract and deploy to your static hosting
   unzip result.zip -d public/data/
   ```

2. **Deploy updated files** to your CDN/static hosting

3. **The frontend automatically** picks up the new recommendations