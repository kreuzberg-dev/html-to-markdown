---
id: fixture_node_emphasis_superscript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>x<sup>2</sup></p>", undefined);
}

void main();

```
