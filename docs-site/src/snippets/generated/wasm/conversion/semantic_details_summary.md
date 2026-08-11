---
id: fixture_wasm_semantic_details_summary
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<details><summary>Click to expand</summary><p>Hidden content here.</p></details>", undefined);
}

void main();

```
