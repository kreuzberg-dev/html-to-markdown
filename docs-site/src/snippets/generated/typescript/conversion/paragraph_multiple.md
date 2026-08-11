---
id: fixture_node_paragraph_multiple
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>First paragraph.</p><p>Second paragraph.</p>", undefined);
}

void main();

```
