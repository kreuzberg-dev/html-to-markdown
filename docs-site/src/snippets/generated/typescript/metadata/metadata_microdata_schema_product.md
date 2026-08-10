```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<html><head><title>Product</title></head><body><div itemscope itemtype=\"https://schema.org/Product\"><h1 itemprop=\"name\">Awesome Widget</h1><span itemprop=\"description\">The best widget on the market</span><span itemprop=\"price\">29.99</span><span itemprop=\"priceCurrency\">USD</span><img itemprop=\"image\" src=\"widget.jpg\" alt=\"Widget\"><span itemprop=\"ratingValue\">4.5</span></div></body></html>", options);
}

void main();

```
