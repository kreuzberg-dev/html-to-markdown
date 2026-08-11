---
id: fixture_wasm_malformed_missing_block_closing_tags
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<div><h1>Title<p>First paragraph<p>Second paragraph</div>", undefined);
}

void main();

```
