---
id: fixture_wasm_conversion_autolink_relative_path_not_autolinked
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<a href=\"/docs/intro.html\">/docs/intro.html</a>", undefined);
}

void main();

```
