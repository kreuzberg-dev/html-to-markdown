---
id: fixture_wasm_link_with_bold_text
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\"><strong>Bold link</strong></a>", undefined);
}

void main();

```
