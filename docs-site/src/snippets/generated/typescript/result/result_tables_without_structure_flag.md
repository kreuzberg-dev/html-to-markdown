---
id: fixture_node_result_tables_without_structure_flag
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", undefined);
}

void main();

```
