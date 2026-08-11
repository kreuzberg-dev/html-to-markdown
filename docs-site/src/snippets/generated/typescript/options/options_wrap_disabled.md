---
id: fixture_node_options_wrap_disabled
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { wrap: false };
  const result = convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options);
}

void main();

```
