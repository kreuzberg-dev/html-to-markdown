---
id: fixture_wasm_code_inline_in_paragraph
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Call the <code>initialize()</code> method first.</p>", undefined);
}

void main();

```
