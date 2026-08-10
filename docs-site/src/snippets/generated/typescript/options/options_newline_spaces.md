```typescript title="TypeScript"
import { ConversionOptions, NewlineStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { newlineStyle: NewlineStyle.Spaces };
  const result = convert("<p>First<br>Second</p>", options);
}

void main();

```
