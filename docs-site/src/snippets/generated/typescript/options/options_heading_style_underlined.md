```typescript title="TypeScript"
import { ConversionOptions, HeadingStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { headingStyle: HeadingStyle.Underlined };
  const result = convert("<h1>Main Title</h1>", options);
}

void main();

```
