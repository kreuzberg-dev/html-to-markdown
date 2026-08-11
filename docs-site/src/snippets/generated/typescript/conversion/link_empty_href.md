---
id: fixture_node_link_empty_href
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"\">No destination</a>", undefined);
}

void main();

```
