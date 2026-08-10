```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [".cookie-banner"] };
  const result = convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options);
}

void main();

```
