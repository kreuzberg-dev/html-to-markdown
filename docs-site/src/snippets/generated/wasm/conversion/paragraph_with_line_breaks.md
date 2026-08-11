---
id: fixture_wasm_paragraph_with_line_breaks
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Line one.<br>Line two.<br>Line three.</p>", undefined);
}

void main();

```
