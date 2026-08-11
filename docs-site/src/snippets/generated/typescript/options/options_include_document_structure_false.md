---
id: fixture_node_options_include_document_structure_false
language: typescript
target: node
level: typecheck
requires: []
side_effect: safe
---

```typescript title="TypeScript"
import { ConversionOptions, convert } from "@xberg-io/html-to-markdown";
function main() {
  const options: ConversionOptions = { includeDocumentStructure: false };
  const result = convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", options);
}

void main();

```
