---
id: fixture_node_options_escape_underscores
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeUnderscores: true };
  const result = convert("<p>The variable_name is defined.</p>", options);
}

void main();

```
