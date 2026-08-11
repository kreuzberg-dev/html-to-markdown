---
id: fixture_wasm_link_with_title
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\" title=\"Example Site\">Example</a>", undefined);
}

void main();

```
