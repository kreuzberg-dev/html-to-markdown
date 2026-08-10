```typescript title="TypeScript"
import { ConversionOptions, HeadingStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { headingStyle: HeadingStyle.AtxClosed };
  const result = convert("<h1>Closed Heading</h1>", options);
}

void main();

```
