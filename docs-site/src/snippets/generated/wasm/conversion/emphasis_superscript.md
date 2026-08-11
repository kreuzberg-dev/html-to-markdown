---
id: fixture_wasm_emphasis_superscript
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>x<sup>2</sup></p>", undefined);
}

void main();

```
