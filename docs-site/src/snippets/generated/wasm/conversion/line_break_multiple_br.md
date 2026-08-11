---
id: fixture_wasm_line_break_multiple_br
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Start.<br><br>End.</p>", undefined);
}

void main();

```
