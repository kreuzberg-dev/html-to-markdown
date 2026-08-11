---
id: fixture_node_link_with_title
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"https://example.com\" title=\"Example Site\">Example</a>", undefined);
}

void main();

```
