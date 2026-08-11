---
id: fixture_node_hidden_content_noscript_element_dropped
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", undefined);
}

void main();

```
