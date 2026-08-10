```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Page</title><link rel=\"canonical\" href=\"https://example.com/canonical-page\"></head><body><p>Content</p></body></html>", options);
}

void main();

```
