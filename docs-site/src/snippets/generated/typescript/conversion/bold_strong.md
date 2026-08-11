---
id: fixture_node_bold_strong
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><strong>bold</strong></p>", undefined);
}

void main();

```
