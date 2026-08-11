---
id: fixture_node_hidden_content_display_none_dropped
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", undefined);
}

void main();

```
