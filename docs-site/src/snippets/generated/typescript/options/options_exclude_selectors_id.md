---
id: fixture_node_options_exclude_selectors_id
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { excludeSelectors: ["#ad-container"] };
  const result = convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options);
}

void main();

```
