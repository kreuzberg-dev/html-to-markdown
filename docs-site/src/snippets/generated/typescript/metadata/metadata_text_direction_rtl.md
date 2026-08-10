```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html lang=\"ar\" dir=\"rtl\"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>", options);
}

void main();

```
