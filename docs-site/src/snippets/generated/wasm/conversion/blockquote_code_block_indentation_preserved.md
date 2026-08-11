---
id: fixture_wasm_blockquote_code_block_indentation_preserved
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<blockquote><pre><code>line1\n    line2 indented</code></pre></blockquote>", undefined);
}

void main();

```
