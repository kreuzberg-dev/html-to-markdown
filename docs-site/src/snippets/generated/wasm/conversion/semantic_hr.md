---
id: fixture_wasm_semantic_hr
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Above</p><hr><p>Below</p>", undefined);
}

void main();

```
