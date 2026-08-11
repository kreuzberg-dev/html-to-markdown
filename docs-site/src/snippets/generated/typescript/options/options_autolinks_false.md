---
id: fixture_node_options_autolinks_false
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { autolinks: false };
  const result = convert("<p><a href='https://example.com'>https://example.com</a></p>", options);
}

void main();

```
