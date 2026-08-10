```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { wrap: false };
  const result = convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options);
}

void main();

```
