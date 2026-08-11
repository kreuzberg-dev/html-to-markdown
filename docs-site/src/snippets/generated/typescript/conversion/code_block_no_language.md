---
id: fixture_node_code_block_no_language
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<pre><code>plain code here</code></pre>", undefined);
}

void main();

```
