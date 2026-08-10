```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Use <code>`backtick` here</code> carefully.</p>", undefined);
}

void main();

```
