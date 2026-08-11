---
id: fixture_node_semantic_sub_superscript
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", undefined);
}

void main();

```
