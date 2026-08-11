---
id: fixture_wasm_blockquote_multiple_paragraphs
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<blockquote><p>First paragraph.</p><p>Second paragraph.</p></blockquote>", undefined);
}

void main();

```
