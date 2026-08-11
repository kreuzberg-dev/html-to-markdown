---
id: fixture_node_line_break_multiple_br
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Start.<br><br>End.</p>", undefined);
}

void main();

```
