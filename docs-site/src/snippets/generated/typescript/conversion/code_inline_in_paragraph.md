---
id: fixture_node_code_inline_in_paragraph
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Call the <code>initialize()</code> method first.</p>", undefined);
}

void main();

```
