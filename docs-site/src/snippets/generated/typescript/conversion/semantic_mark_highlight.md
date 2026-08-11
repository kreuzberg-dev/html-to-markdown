---
id: fixture_node_semantic_mark_highlight
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>This is <mark>highlighted text</mark> in a sentence.</p>", undefined);
}

void main();

```
