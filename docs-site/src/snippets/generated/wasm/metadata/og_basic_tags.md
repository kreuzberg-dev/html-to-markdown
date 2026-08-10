```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<html><head><title>Fallback Title</title><meta property=\"og:title\" content=\"OG Title\"><meta property=\"og:description\" content=\"OG description text.\"><meta property=\"og:image\" content=\"https://example.com/image.jpg\"></head><body><p>Content</p></body></html>", undefined);
}

void main();

```
