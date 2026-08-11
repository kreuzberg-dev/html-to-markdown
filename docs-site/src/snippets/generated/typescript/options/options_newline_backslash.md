---
id: fixture_node_options_newline_backslash
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, NewlineStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { newlineStyle: NewlineStyle.Backslash };
  const result = convert("<p>Line one<br>Line two</p>", options);
}

void main();

```
