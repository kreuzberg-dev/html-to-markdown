---
id: fixture_node_heading_h5
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h5>Heading 5</h5>", undefined);
}

void main();

```
