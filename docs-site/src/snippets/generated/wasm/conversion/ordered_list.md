---
id: fixture_wasm_ordered_list
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<ol><li>First</li><li>Second</li><li>Third</li></ol>", undefined);
}

void main();

```
