---
id: fixture_wasm_html_comments_only
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<!-- This is a comment --><!-- Another comment -->", undefined);
}

void main();

```
