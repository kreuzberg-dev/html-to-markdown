---
id: fixture_wasm_malformed_bogus_comment_triple_dash
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<h1>One</h1>\n<!-- /// --->\n<p>Two</p>", undefined);
}

void main();

```
