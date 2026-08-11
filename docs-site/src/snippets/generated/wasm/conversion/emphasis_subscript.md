---
id: fixture_wasm_emphasis_subscript
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>H<sub>2</sub>O</p>", undefined);
}

void main();

```
