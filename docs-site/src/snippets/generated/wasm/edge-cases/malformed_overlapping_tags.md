---
id: fixture_wasm_malformed_overlapping_tags
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><b><i>bold and italic</b></i></p>", undefined);
}

void main();

```
