---
id: fixture_wasm_heading_h3
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h3>Heading 3</h3>", undefined);
}

void main();

```
