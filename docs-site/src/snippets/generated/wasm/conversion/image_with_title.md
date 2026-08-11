---
id: fixture_wasm_image_with_title
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<img src=\"chart.png\" alt=\"Sales chart\" title=\"Q3 Sales\">", undefined);
}

void main();

```
