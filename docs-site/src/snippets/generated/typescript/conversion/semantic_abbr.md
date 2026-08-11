---
id: fixture_node_semantic_abbr
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", undefined);
}

void main();

```
