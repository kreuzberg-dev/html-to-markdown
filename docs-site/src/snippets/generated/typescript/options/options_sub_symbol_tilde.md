---
id: fixture_node_options_sub_symbol_tilde
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { subSymbol: "~" };
  const result = convert("<p>H<sub>2</sub>O</p>", options);
}

void main();

```
