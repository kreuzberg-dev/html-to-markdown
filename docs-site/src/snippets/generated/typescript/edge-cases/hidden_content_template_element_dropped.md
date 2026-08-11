---
id: fixture_node_hidden_content_template_element_dropped
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>visible</p><template><p>secret template text</p></template><p>also visible</p>", undefined);
}

void main();

```
