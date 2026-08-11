---
id: fixture_wasm_conversion_autolink_https_url
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"https://example.com\">https://example.com</a>", undefined);
}

void main();

```
