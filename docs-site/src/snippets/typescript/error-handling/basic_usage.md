```typescript
import { convert } from "@xberg-io/html-to-markdown";

// Binary data (detected via magic bytes) is rejected before parsing.
const html = "%PDF-1.4 not actually HTML";

try {
  const result = convert(html);
  console.log(result.content ?? "");
} catch (error) {
  // The native binding surfaces conversion failures as a standard Error.
  console.error("conversion failed:", (error as Error).message);
}
```
