```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractImages: true };
  const result = convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options);
}

void main();

```
