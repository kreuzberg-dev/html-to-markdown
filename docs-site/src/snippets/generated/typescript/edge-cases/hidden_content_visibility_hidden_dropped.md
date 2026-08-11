---
id: fixture_node_hidden_content_visibility_hidden_dropped
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", undefined);
}

void main();

```
