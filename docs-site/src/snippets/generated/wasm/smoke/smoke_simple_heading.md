---
id: fixture_wasm_smoke_simple_heading
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h1>Title</h1>", undefined);
}

void main();

```
