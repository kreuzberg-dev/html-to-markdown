```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<p>See <a href=\"https://example.com\">Example</a> for details.</p>", options);
}

void main();

```
