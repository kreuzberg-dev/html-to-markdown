```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Gallery</title></head><body><img src=\"https://example.com/photo1.jpg\" alt=\"Photo 1\"><img src=\"https://example.com/photo2.png\" alt=\"Photo 2\"><img src=\"/local/image.webp\" alt=\"Local image\"></body></html>", options);
}

void main();

```
