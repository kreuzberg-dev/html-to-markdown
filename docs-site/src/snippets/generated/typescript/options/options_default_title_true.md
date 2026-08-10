```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { defaultTitle: true };
  const result = convert("<p><a href='https://example.com'>Link</a></p>", options);
}

void main();

```
