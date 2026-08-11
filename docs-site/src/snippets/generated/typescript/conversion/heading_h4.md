---
id: fixture_node_heading_h4
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h4>Heading 4</h4>", undefined);
}

void main();

```
