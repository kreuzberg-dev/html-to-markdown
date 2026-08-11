---
id: fixture_node_result_warnings_empty_for_malformed_html
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", undefined);
}

void main();

```
