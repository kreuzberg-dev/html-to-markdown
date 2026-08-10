```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { maxDepth: 0 };
  const result = convert("<p>Hello</p>", options);
}

void main();

```
