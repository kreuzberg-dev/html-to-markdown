---
id: fixture_node_image_linked
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", undefined);
}

void main();

```
