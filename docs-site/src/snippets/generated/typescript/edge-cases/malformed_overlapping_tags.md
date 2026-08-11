---
id: fixture_node_malformed_overlapping_tags
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><b><i>bold and italic</b></i></p>", undefined);
}

void main();

```
