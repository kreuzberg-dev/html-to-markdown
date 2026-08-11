---
id: fixture_wasm_emphasis_mark_highlight
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><mark>highlighted</mark></p>", undefined);
}

void main();

```
