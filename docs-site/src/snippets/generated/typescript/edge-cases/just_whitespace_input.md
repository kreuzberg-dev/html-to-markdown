---
id: fixture_node_just_whitespace_input
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("   ", undefined);
}

void main();

```
