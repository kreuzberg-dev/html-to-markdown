---
id: fixture_node_metadata_link_type_anchor_classified
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { extractMetadata: true };
  const result = convert("<p>Jump to <a href=\"#section\">section</a> below.</p>", options);
}

void main();

```
