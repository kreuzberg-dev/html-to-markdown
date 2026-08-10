```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { stripTags: ["div", "span"] };
  const result = convert("<div class='wrapper'><p>Inside div</p></div><p>Outside <span class='hl'>span text</span></p>", options);
}

void main();

```
