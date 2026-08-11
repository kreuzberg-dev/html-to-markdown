---
id: fixture_node_smoke_simple_heading
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h1>Title</h1>", undefined);
}

void main();

```
