```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { subSymbol: "~" };
  const result = convert("<p>H<sub>2</sub>O</p>", options);
}

void main();

```
