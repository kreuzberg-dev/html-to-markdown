```typescript title="TypeScript"
import { ConversionOptions, OutputFormat, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { outputFormat: OutputFormat.Plain };
  const result = convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options);
}

void main();

```
