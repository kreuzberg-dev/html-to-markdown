---
id: fixture_wasm_code_with_backticks_in_content
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Use <code>`backtick` here</code> carefully.</p>", undefined);
}

void main();

```
