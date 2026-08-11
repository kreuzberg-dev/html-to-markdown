---
id: fixture_node_conversion_autolink_mailto
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"mailto:a@b.com\">a@b.com</a>", undefined);
}

void main();

```
