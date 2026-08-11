---
id: fixture_wasm_heading_h2
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h2>Heading 2</h2>", undefined);
}

void main();

```
