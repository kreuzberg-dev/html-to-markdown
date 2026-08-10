```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", undefined);
}

void main();

```
