---
id: fixture_wasm_options_max_depth_default_unlimited
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<div><div><div><div><p>Deep content</p></div></div></div></div>", undefined);
}

void main();

```
