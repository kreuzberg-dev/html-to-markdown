---
id: fixture_wasm_semantic_sub_superscript
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>H<sub>2</sub>O and E=mc<sup>2</sup></p>", undefined);
}

void main();

```
