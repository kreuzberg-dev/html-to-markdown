---
id: fixture_wasm_semantic_article
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<article><h2>Article Title</h2><p>Article body.</p></article>", undefined);
}

void main();

```
