```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { convertAsInline: true };
  const result = convert("<p>One</p><p>Two</p>", options);
}

void main();

```
