---
id: fixture_node_italic_em
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><em>italic</em></p>", undefined);
}

void main();

```
