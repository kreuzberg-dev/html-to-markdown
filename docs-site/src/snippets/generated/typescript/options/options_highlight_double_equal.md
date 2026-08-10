```typescript title="TypeScript"
import { ConversionOptions, HighlightStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { highlightStyle: HighlightStyle.DoubleEqual };
  const result = convert("<p>Text with <mark>highlighted</mark> here.</p>", options);
}

void main();

```
