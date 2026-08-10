```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeAsterisks: true };
  const result = convert("<p>Use 2*3 = 6 in math.</p>", options);
}

void main();

```
