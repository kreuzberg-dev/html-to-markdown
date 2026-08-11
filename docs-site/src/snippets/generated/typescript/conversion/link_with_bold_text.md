---
id: fixture_node_link_with_bold_text
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", undefined);
}

void main();

```
