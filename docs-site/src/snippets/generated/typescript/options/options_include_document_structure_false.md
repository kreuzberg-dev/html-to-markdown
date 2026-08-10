```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { includeDocumentStructure: false };
  const result = convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", options);
}

void main();

```
