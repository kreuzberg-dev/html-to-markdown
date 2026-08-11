---
id: fixture_node_heading_h3
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h3>Heading 3</h3>", undefined);
}

void main();

```
