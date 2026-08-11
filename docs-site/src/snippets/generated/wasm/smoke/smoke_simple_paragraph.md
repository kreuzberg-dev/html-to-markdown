---
id: fixture_wasm_smoke_simple_paragraph
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Hello World</p>", undefined);
}

void main();

```
