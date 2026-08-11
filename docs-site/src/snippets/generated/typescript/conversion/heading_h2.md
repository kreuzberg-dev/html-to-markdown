---
id: fixture_node_heading_h2
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h2>Heading 2</h2>", undefined);
}

void main();

```
