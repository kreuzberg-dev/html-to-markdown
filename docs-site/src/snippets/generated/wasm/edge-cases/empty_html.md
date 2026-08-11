---
id: fixture_wasm_empty_html
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<html><head></head><body></body></html>", undefined);
}

void main();

```
