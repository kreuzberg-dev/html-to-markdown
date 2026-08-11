---
id: fixture_wasm_result_tables_without_structure_flag
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<table><tr><th>X</th></tr><tr><td>Y</td></tr></table>", undefined);
}

void main();

```
