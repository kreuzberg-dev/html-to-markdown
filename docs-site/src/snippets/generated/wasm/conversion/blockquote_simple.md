---
id: fixture_wasm_blockquote_simple
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<blockquote><p>Quote text</p></blockquote>", undefined);
}

void main();

```
