---
id: fixture_wasm_hidden_content_visibility_hidden_dropped
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>visible</p><span style=\"visibility:hidden\">secret hidden span</span><p>also visible</p>", undefined);
}

void main();

```
