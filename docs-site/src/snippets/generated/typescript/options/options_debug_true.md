```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { debug: true };
  const result = convert("<p>Debug test</p>", options);
}

void main();

```
