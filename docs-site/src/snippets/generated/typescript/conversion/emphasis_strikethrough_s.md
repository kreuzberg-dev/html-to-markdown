---
id: fixture_node_emphasis_strikethrough_s
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p><s>strikethrough</s></p>", undefined);
}

void main();

```
