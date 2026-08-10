```typescript title="TypeScript"
import { ConversionOptions, LinkStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { linkStyle: LinkStyle.Reference };
  const result = convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options);
}

void main();

```
