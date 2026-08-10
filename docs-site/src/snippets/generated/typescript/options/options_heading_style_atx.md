```typescript title="TypeScript"
import { ConversionOptions, HeadingStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { headingStyle: HeadingStyle.Atx };
  const result = convert("<h1>Title</h1><h2>Subtitle</h2>", options);
}

void main();

```
