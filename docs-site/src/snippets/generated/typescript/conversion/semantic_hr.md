---
id: fixture_node_semantic_hr
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Above</p><hr><p>Below</p>", undefined);
}

void main();

```
