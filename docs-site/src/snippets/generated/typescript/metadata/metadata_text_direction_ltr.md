```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html lang=\"en\" dir=\"ltr\"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>", options);
}

void main();

```
