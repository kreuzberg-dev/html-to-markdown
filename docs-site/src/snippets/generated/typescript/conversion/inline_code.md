---
id: fixture_node_inline_code
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Use <code>console.log()</code> to debug</p>", undefined);
}

void main();

```
