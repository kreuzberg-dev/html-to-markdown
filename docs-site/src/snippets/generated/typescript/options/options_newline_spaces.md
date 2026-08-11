---
id: fixture_node_options_newline_spaces
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, NewlineStyle, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { newlineStyle: NewlineStyle.Spaces };
  const result = convert("<p>First<br>Second</p>", options);
}

void main();

```
