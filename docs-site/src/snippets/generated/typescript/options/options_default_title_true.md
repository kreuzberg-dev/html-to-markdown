---
id: fixture_node_options_default_title_true
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { defaultTitle: true };
  const result = convert("<p><a href='https://example.com'>Link</a></p>", options);
}

void main();

```
