---
id: fixture_wasm_paragraph_nested_divs
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<div><div><p>Nested text</p></div></div>", undefined);
}

void main();

```
