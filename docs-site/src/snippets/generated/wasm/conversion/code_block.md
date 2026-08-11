---
id: fixture_wasm_code_block
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<pre><code class=\"language-python\">print('hello')</code></pre>", undefined);
}

void main();

```
