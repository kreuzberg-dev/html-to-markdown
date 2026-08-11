---
id: fixture_node_hidden_content_aria_hidden_still_rendered
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { convert } from "@xberg-io/html-to-markdown";
function main() {
  const result = convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", undefined);
}

void main();

```
