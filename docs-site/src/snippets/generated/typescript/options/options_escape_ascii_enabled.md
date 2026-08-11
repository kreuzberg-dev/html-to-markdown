---
id: fixture_node_options_escape_ascii_enabled
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { escapeAscii: true };
  const result = convert("<p>Text with # hash and [brackets] and * star</p>", options);
}

void main();

```
