---
id: fixture_node_options_list_custom_bullets
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { bullets: "*" };
  const result = convert("<ul><li>Item A</li><li>Item B</li></ul>", options);
}

void main();

```
