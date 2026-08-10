```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { maxDepth: 3 };
  const result = convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options);
}

void main();

```
