---
id: fixture_node_link_image_inside
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", undefined);
}

void main();

```
