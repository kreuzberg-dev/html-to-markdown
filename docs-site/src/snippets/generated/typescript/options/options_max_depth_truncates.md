---
id: fixture_node_options_max_depth_truncates
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { maxDepth: 3 };
  const result = convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options);
}

void main();

```
