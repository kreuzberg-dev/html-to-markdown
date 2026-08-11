---
id: fixture_node_html_comments_only
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<!-- This is a comment --><!-- Another comment -->", undefined);
}

void main();

```
