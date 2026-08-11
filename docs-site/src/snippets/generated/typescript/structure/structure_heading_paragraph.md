---
id: fixture_node_structure_heading_paragraph
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { includeDocumentStructure: true };
  const result = convert("<h1>Title</h1><p>A paragraph of text.</p>", options);
}

void main();

```
