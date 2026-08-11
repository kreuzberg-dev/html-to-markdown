---
id: fixture_node_options_exclude_selectors_attribute
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: ["[role='complementary']"] };
  const result = convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options);
}

void main();

```
