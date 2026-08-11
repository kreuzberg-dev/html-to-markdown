---
id: fixture_wasm_script_tags_only
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<html><head><script>alert('xss')</script></head><body><script>document.write('hello')</script></body></html>", undefined);
}

void main();

```
