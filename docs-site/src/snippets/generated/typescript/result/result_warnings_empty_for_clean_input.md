---
id: fixture_node_result_warnings_empty_for_clean_input
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<h1>Title</h1><p>Clean content with <a href='https://example.com'>a link</a>.</p>", undefined);
}

void main();

```
