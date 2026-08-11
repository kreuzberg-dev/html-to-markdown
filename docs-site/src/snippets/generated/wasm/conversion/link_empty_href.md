---
id: fixture_wasm_link_empty_href
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"\">No destination</a>", undefined);
}

void main();

```
