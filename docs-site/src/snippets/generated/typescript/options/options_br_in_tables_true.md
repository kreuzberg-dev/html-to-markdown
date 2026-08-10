```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { brInTables: true };
  const result = convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options);
}

void main();

```
