---
id: fixture_wasm_bold_and_italic
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><strong><em>both</em></strong></p>", undefined);
}

void main();

```
