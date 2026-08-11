---
id: fixture_node_malformed_unclosed_paragraph
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>This paragraph is never closed", undefined);
}

void main();

```
