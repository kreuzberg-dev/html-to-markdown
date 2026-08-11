---
id: fixture_wasm_emphasis_strikethrough_s
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><s>strikethrough</s></p>", undefined);
}

void main();

```
