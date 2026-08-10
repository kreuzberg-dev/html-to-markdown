```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { encoding: "utf-8" };
  const result = convert("<p>Café naïve résumé</p>", options);
}

void main();

```
