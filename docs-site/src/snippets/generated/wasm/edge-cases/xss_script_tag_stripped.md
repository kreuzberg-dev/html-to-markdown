---
id: fixture_wasm_xss_script_tag_stripped
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Safe content.</p><script>alert('xss')</script><p>More safe content.</p>", undefined);
}

void main();

```
