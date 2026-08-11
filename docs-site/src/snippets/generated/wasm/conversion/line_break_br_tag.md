---
id: fixture_wasm_line_break_br_tag
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>First line.<br>Second line.</p>", undefined);
}

void main();

```
