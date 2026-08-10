```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>My Page</title></head><body><p>Content</p></body></html>", options);
}

void main();

```
