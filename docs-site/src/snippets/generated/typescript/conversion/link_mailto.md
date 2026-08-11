---
id: fixture_node_link_mailto
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"mailto:user@example.com\">Email us</a>", undefined);
}

void main();

```
