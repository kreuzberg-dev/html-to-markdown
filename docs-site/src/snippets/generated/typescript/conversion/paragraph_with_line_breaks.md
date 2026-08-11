---
id: fixture_node_paragraph_with_line_breaks
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Line one.<br>Line two.<br>Line three.</p>", undefined);
}

void main();

```
