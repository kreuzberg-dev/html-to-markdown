```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<blockquote><p>Quote intro:</p><ul><li>Point one</li><li>Point two</li></ul></blockquote>", undefined);
}

void main();

```
