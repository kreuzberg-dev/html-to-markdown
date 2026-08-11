---
id: fixture_node_ordered_list
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", undefined);
}

void main();

```
