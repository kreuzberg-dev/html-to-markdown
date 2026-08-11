---
id: fixture_wasm_heading_h4
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h4>Heading 4</h4>", undefined);
}

void main();

```
