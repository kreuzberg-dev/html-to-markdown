---
id: fixture_node_xss_script_tag_stripped
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", undefined);
}

void main();

```
