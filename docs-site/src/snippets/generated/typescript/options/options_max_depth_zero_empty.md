---
id: fixture_node_options_max_depth_zero_empty
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { maxDepth: 0 };
  const result = convert("<p>Hello</p>", options);
}

void main();

```
