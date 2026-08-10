```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { autolinks: false };
  const result = convert("<p><a href='https://example.com'>https://example.com</a></p>", options);
}

void main();

```
