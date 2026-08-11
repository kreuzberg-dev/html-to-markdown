---
id: fixture_node_image_with_title
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", undefined);
}

void main();

```
