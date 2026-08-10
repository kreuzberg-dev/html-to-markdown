```typescript title="TypeScript"
import { ConversionOptions, HeadingStyle, OutputFormat, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { headingStyle: HeadingStyle.Atx, outputFormat: OutputFormat.Markdown };
  const result = convert("<h1>Title</h1><p>Some text.</p>", options);
}

void main();

```
