---
id: fixture_node_code_block
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<pre><code class=\"language-python\">print('hello')</code></pre>", undefined);
}

void main();

```
