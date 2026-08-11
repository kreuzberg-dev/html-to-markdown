---
id: fixture_wasm_code_block_no_language
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<pre><code>plain code here</code></pre>", undefined);
}

void main();

```
