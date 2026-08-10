```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", undefined);
}

void main();

```
