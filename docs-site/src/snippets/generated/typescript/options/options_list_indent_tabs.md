```typescript title="TypeScript"
import { ConversionOptions, ListIndentType, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { listIndentType: ListIndentType.Tabs };
  const result = convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options);
}

void main();

```
