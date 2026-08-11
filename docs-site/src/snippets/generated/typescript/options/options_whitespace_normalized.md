---
id: fixture_node_options_whitespace_normalized
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, WhitespaceMode, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { whitespaceMode: WhitespaceMode.Normalized };
  const result = convert("<p>Text   with    extra   spaces.</p>", options);
}

void main();

```
