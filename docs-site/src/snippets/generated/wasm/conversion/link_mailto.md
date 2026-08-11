---
id: fixture_wasm_link_mailto
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"mailto:user@example.com\">Email us</a>", undefined);
}

void main();

```
