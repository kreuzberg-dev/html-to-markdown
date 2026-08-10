```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Page</title><meta name=\"author\" content=\"Jane Doe\"></head><body><p>Content</p></body></html>", options);
}

void main();

```
