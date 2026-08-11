---
id: fixture_node_metadata_microdata_schema_article
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Article</title></head><body><article itemscope itemtype=\"https://schema.org/Article\"><h1 itemprop=\"headline\">Breaking News Today</h1><span itemprop=\"author\">Jane Reporter</span><span itemprop=\"datePublished\">2024-04-22</span><div itemprop=\"articleBody\"><p>The article content goes here with important information about the breaking news story.</p></div></article></body></html>", options);
}

void main();

```
