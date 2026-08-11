---
id: fixture_node_emphasis_strikethrough_del
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><del>deleted text</del></p>", undefined);
}

void main();

```
