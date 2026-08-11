---
id: fixture_node_empty_html
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<html><head></head><body></body></html>", undefined);
}

void main();

```
