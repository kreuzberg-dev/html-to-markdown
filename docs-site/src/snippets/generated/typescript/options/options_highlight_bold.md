```typescript title="TypeScript"
import { ConversionOptions, HighlightStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { highlightStyle: HighlightStyle.Bold };
  const result = convert("<p>Text with <mark>highlighted</mark> text.</p>", options);
}

void main();

```
