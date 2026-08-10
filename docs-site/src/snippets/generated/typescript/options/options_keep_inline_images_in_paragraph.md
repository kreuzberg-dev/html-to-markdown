```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { keepInlineImagesIn: ["p"] };
  const result = convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options);
}

void main();

```
