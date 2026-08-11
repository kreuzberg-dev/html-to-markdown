---
id: fixture_node_options_strong_em_underscore
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { strongEmSymbol: "_" };
  const result = convert("<p><strong>bold</strong> and <em>italic</em></p>", options);
}

void main();

```
