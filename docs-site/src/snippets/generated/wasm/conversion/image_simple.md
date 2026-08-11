---
id: fixture_wasm_image_simple
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<img src=\"photo.jpg\" alt=\"A photo\">", undefined);
}

void main();

```
