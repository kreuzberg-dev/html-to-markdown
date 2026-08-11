---
id: fixture_node_heading_h1
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h1>Heading 1</h1>", undefined);
}

void main();

```
