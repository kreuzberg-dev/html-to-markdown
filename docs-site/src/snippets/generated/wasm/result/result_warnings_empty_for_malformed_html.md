---
id: fixture_wasm_result_warnings_empty_for_malformed_html
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", undefined);
}

void main();

```
