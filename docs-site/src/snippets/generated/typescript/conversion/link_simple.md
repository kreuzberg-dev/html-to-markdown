---
id: fixture_node_link_simple
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"https://example.com\">Example</a>", undefined);
}

void main();

```
