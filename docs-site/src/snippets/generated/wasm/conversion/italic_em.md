---
id: fixture_wasm_italic_em
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><em>italic</em></p>", undefined);
}

void main();

```
