```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeUnderscores: true };
  const result = convert("<p>The variable_name is defined.</p>", options);
}

void main();

```
