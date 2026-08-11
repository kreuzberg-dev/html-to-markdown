---
id: fixture_wasm_hidden_content_aria_hidden_still_rendered
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>visible</p><div aria-hidden=\"true\">still shown</div><p>also visible</p>", undefined);
}

void main();

```
