---
id: fixture_node_blockquote_nested_list_indentation_preserved
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<blockquote><ul><li>item a<ul><li>sub a1</li></ul></li></ul></blockquote>", undefined);
}

void main();

```
