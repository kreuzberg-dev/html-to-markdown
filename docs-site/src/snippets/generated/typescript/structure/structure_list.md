```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { includeDocumentStructure: true };
  const result = convert("<p>Items:</p><ul><li>Alpha</li><li>Beta</li><li>Gamma</li></ul>", options);
}

void main();

```
