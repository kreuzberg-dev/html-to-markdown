---
id: fixture_node_options_debug_true
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { debug: true };
  const result = convert("<p>Debug test</p>", options);
}

void main();

```
