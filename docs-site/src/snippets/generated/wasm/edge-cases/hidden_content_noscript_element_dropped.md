---
id: fixture_wasm_hidden_content_noscript_element_dropped
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>visible</p><noscript><p>secret noscript text</p></noscript><p>also visible</p>", undefined);
}

void main();

```
