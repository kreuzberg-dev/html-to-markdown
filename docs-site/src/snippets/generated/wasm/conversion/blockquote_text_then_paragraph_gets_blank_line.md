---
id: fixture_wasm_blockquote_text_then_paragraph_gets_blank_line
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<blockquote>Just text, then <p>a paragraph</p></blockquote>", undefined);
}

void main();

```
