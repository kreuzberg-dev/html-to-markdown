```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { listIndentWidth: 4 };
  const result = convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", options);
}

void main();

```
