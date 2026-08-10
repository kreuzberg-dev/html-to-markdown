```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [] };
  const result = convert("<p>Hello world</p>", options);
}

void main();

```
