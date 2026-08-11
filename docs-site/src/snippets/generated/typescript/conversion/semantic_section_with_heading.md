---
id: fixture_node_semantic_section_with_heading
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", undefined);
}

void main();

```
