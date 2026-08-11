---
id: fixture_node_malformed_missing_block_closing_tags
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", undefined);
}

void main();

```
