---
id: fixture_wasm_emphasis_strikethrough_del
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><del>deleted text</del></p>", undefined);
}

void main();

```
