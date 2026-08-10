```typescript title="TypeScript"
import { ConversionOptions, NewlineStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { newlineStyle: NewlineStyle.Backslash };
  const result = convert("<p>Line one<br>Line two</p>", options);
}

void main();

```
