---
id: fixture_node_image_no_alt
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<img src=\"banner.jpg\">", undefined);
}

void main();

```
