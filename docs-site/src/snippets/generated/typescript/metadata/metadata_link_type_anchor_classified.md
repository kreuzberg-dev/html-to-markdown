```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<p>Jump to <a href=\"#section\">section</a> below.</p>", options);
}

void main();

```
