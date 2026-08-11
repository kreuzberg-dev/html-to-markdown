---
id: fixture_wasm_semantic_mark_highlight
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>This is <mark>highlighted text</mark> in a sentence.</p>", undefined);
}

void main();

```
