```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [".wrapper"] };
  const result = convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options);
}

void main();

```
