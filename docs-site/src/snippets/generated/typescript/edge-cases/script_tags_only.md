---
id: fixture_node_script_tags_only
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", undefined);
}

void main();

```
