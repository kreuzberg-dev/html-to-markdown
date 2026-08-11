---
id: fixture_wasm_unordered_list
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<ul><li>Item 1</li><li>Item 2</li><li>Item 3</li></ul>", undefined);
}

void main();

```
