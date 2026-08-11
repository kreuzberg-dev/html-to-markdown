---
id: fixture_node_options_exclude_selectors_empty_noop
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [] };
  const result = convert("<p>Hello world</p>", options);
}

void main();

```
