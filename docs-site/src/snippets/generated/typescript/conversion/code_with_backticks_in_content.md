---
id: fixture_node_code_with_backticks_in_content
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Use <code>`backtick` here</code> carefully.</p>", undefined);
}

void main();

```
