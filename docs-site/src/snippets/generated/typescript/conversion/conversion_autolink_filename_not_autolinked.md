---
id: fixture_node_conversion_autolink_filename_not_autolinked
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"foobar.png\">foobar.png</a>", undefined);
}

void main();

```
