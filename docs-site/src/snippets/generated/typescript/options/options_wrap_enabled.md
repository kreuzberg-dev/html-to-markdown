```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { wrap: true, wrapWidth: 40 };
  const result = convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options);
}

void main();

```
