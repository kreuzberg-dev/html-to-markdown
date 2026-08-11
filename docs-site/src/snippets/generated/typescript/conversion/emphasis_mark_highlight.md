---
id: fixture_node_emphasis_mark_highlight
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><mark>highlighted</mark></p>", undefined);
}

void main();

```
