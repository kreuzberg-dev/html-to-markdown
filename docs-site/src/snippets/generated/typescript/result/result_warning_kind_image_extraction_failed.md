---
id: fixture_node_result_warning_kind_image_extraction_failed
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractImages: true };
  const result = convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options);
}

void main();

```
