```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", undefined);
}

void main();

```
