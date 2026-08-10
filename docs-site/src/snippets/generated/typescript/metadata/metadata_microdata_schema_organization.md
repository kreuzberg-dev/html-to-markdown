```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Company</title></head><body><div itemscope itemtype=\"https://schema.org/Organization\"><span itemprop=\"name\">Acme Corp</span><span itemprop=\"foundingDate\">2020</span><span itemprop=\"url\">https://acmecorp.example.com</span><span itemprop=\"logo\">https://acmecorp.example.com/logo.png</span></div></body></html>", options);
}

void main();

```
