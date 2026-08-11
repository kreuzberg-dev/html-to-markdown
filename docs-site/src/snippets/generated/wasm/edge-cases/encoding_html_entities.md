---
id: fixture_wasm_encoding_html_entities
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>&amp; &lt; &gt; &nbsp; &quot; &apos;</p>", undefined);
}

void main();

```
