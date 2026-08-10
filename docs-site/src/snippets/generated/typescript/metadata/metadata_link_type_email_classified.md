```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<p>Contact <a href=\"mailto:hello@example.com\">us</a> directly.</p>", options);
}

void main();

```
