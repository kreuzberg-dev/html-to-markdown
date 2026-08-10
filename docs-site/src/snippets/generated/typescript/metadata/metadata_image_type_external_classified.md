```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<p><img src=\"https://example.com/photo.jpg\" alt=\"A photo\"></p>", options);
}

void main();

```
