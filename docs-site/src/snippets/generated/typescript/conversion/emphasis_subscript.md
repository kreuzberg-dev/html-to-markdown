---
id: fixture_node_emphasis_subscript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>H<sub>2</sub>O</p>", undefined);
}

void main();

```
