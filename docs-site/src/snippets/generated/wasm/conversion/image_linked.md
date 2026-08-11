---
id: fixture_wasm_image_linked
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\"><img src=\"icon.png\" alt=\"Icon\"></a>", undefined);
}

void main();

```
