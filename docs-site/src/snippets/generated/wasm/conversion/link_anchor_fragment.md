---
id: fixture_wasm_link_anchor_fragment
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"#section\">Jump to section</a>", undefined);
}

void main();

```
