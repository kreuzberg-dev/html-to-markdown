---
id: fixture_wasm_semantic_abbr
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>The <abbr title=\"World Wide Web\">WWW</abbr> is global.</p>", undefined);
}

void main();

```
