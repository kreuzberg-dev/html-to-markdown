---
id: fixture_node_malformed_bogus_comment_triple_dash
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", undefined);
}

void main();

```
