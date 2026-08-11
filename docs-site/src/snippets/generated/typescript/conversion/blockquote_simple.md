---
id: fixture_node_blockquote_simple
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<blockquote><p>Quote text</p></blockquote>", undefined);
}

void main();

```
