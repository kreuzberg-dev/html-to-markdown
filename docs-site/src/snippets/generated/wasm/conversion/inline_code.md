---
id: fixture_wasm_inline_code
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Use <code>console.log()</code> to debug</p>", undefined);
}

void main();

```
