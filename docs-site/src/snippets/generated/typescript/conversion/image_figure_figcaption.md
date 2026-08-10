```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<figure><img src=\"sunset.jpg\" alt=\"A sunset\"><figcaption>Beautiful sunset over the ocean</figcaption></figure>", undefined);
}

void main();

```
