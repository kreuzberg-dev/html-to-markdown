```typescript title="TypeScript"
import { ConversionOptions, OutputFormat, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { outputFormat: OutputFormat.Djot };
  const result = convert("<p>Simple paragraph.</p>", options);
}

void main();

```
