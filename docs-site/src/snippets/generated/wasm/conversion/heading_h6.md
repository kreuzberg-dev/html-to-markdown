---
id: fixture_wasm_heading_h6
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h6>Heading 6</h6>", undefined);
}

void main();

```
