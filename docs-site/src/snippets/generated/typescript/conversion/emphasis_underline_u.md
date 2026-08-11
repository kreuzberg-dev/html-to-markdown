---
id: fixture_node_emphasis_underline_u
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><u>underlined</u></p>", undefined);
}

void main();

```
