---
id: fixture_wasm_emphasis_underline_u
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><u>underlined</u></p>", undefined);
}

void main();

```
