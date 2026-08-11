---
id: fixture_node_options_strip_newlines
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { stripNewlines: true };
  const result = convert("<p>First paragraph.</p><p>Second paragraph.</p>", options);
}

void main();

```
