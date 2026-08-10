```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { stripNewlines: true };
  const result = convert("<p>First paragraph.</p><p>Second paragraph.</p>", options);
}

void main();

```
