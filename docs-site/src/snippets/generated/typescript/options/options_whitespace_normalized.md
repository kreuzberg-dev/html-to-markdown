```typescript title="TypeScript"
import { ConversionOptions, WhitespaceMode, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { whitespaceMode: WhitespaceMode.Normalized };
  const result = convert("<p>Text   with    extra   spaces.</p>", options);
}

void main();

```
