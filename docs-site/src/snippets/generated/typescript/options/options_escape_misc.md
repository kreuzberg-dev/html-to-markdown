---
id: fixture_node_options_escape_misc
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeMisc: true };
  const result = convert("<p>Use # and | and ~ in text.</p>", options);
}

void main();

```
