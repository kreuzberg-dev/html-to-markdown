---
id: fixture_wasm_heading_h5
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h5>Heading 5</h5>", undefined);
}

void main();

```
