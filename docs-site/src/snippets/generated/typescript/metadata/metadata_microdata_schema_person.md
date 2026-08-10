```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Contact</title></head><body><div itemscope itemtype=\"https://schema.org/Person\"><span itemprop=\"name\">John Smith</span><span itemprop=\"email\">john@example.com</span><span itemprop=\"telephone\">+1-555-0100</span></div></body></html>", options);
}

void main();

```
