---
id: fixture_node_paragraph_nested_divs
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<div><div><p>Nested text</p></div></div>", undefined);
}

void main();

```
