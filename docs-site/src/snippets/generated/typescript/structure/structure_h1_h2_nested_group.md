```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { includeDocumentStructure: true };
  const result = convert("<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", options);
}

void main();

```
