---
id: fixture_wasm_image_no_alt
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<img src=\"banner.jpg\">", undefined);
}

void main();

```
