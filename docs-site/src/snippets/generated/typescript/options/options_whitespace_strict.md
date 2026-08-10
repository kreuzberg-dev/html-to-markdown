```typescript title="TypeScript"
import { ConversionOptions, WhitespaceMode, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { whitespaceMode: WhitespaceMode.Strict };
  const result = convert("<p>Preserved   spacing.</p>", options);
}

void main();

```
