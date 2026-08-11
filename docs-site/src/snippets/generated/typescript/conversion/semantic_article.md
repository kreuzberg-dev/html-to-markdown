---
id: fixture_node_semantic_article
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<article><h2>Article Title</h2><p>Article body.</p></article>", undefined);
}

void main();

```
