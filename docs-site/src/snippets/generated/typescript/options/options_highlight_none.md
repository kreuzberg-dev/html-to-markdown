```typescript title="TypeScript"
import { ConversionOptions, HighlightStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { highlightStyle: HighlightStyle.None };
  const result = convert("<p>Text with <mark>plain</mark> content.</p>", options);
}

void main();

```
