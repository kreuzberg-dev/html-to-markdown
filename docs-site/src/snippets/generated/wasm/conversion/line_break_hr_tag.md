---
id: fixture_wasm_line_break_hr_tag
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Before rule.</p><hr><p>After rule.</p>", undefined);
}

void main();

```
