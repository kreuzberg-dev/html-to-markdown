---
id: fixture_node_options_escape_asterisks
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeAsterisks: true };
  const result = convert("<p>Use 2*3 = 6 in math.</p>", options);
}

void main();

```
