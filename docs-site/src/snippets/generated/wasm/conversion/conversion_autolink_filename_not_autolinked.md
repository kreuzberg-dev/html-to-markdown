---
id: fixture_wasm_conversion_autolink_filename_not_autolinked
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"foobar.png\">foobar.png</a>", undefined);
}

void main();

```
