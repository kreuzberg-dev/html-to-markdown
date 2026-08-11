---
id: fixture_node_options_exclude_selectors_class
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: [".cookie-banner"] };
  const result = convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options);
}

void main();

```
