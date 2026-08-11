---
id: fixture_wasm_paragraph_multiple
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>First paragraph.</p><p>Second paragraph.</p>", undefined);
}

void main();

```
