---
id: fixture_node_options_convert_as_inline
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { convertAsInline: true };
  const result = convert("<p>One</p><p>Two</p>", options);
}

void main();

```
