---
id: fixture_wasm_hidden_content_template_element_dropped
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>visible</p><template><p>secret template text</p></template><p>also visible</p>", undefined);
}

void main();

```
