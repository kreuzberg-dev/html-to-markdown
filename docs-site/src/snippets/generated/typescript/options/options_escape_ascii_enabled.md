```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeAscii: true };
  const result = convert("<p>Text with # hash and [brackets] and * star</p>", options);
}

void main();

```
