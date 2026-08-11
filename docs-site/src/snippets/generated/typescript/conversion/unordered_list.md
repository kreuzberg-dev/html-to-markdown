---
id: fixture_node_unordered_list
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", undefined);
}

void main();

```
