---
id: fixture_wasm_semantic_section_with_heading
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<section><h3>Section Heading</h3><p>Section content.</p></section>", undefined);
}

void main();

```
