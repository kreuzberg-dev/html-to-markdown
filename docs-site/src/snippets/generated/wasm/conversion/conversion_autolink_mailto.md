---
id: fixture_wasm_conversion_autolink_mailto
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"mailto:a@b.com\">a@b.com</a>", undefined);
}

void main();

```
