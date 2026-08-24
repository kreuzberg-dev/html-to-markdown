---
target: node
---

```typescript
import { convert, ConversionOptions } from "@xberg-io/html-to-markdown";

const options: ConversionOptions = { extractMetadata: true };
const result = convert("<h1>Title</h1><p>Content</p>", options);

console.log(result.content); // Converted markdown
console.log(result.metadata?.document); // Document metadata (title, description, etc.)
console.log(result.metadata?.headers); // Header elements (h1-h6)
console.log(result.metadata?.links); // Extracted links
console.log(result.metadata?.images); // Extracted images
```
