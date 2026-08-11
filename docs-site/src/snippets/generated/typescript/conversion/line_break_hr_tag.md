---
id: fixture_node_line_break_hr_tag
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Before rule.</p><hr><p>After rule.</p>", undefined);
}

void main();

```
