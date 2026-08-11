---
id: fixture_node_link_anchor_fragment
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"#section\">Jump to section</a>", undefined);
}

void main();

```
