---
id: fixture_wasm_heading_h1
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h1>Heading 1</h1>", undefined);
}

void main();

```
