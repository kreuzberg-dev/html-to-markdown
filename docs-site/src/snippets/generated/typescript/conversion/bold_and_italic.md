---
id: fixture_node_bold_and_italic
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><strong><em>both</em></strong></p>", undefined);
}

void main();

```
