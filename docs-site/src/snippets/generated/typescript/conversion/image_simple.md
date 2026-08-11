---
id: fixture_node_image_simple
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<img src=\"photo.jpg\" alt=\"A photo\">", undefined);
}

void main();

```
