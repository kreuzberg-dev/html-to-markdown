---
id: fixture_node_line_break_br_tag
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>First line.<br>Second line.</p>", undefined);
}

void main();

```
