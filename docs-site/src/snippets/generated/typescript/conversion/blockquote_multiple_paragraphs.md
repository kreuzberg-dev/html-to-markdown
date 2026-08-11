---
id: fixture_node_blockquote_multiple_paragraphs
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", undefined);
}

void main();

```
