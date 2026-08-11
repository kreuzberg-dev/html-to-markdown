---
id: fixture_node_conversion_autolink_relative_path_not_autolinked
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>", undefined);
}

void main();

```
