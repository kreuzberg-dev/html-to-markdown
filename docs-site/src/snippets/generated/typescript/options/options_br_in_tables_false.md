```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { brInTables: false };
  const result = convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options);
}

void main();

```
