---
id: fixture_wasm_link_image_inside
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\"><img src=\"logo.png\" alt=\"Logo\"></a>", undefined);
}

void main();

```
