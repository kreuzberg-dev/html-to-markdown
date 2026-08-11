---
id: fixture_node_blockquote_code_block_indentation_preserved
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", undefined);
}

void main();

```
