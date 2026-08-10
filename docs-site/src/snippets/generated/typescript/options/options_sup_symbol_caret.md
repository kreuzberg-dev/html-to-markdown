```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { supSymbol: "^" };
  const result = convert("<p>x<sup>2</sup></p>", options);
}

void main();

```
