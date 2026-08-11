---
id: fixture_wasm_hidden_content_display_none_dropped
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>visible</p><div style=\"display:none\">secret hidden text</div><p>also visible</p>", undefined);
}

void main();

```
