---
id: fixture_wasm_options_br_in_tables_false
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.brInTables = false; return _u0; })();
  const result = convert("<table><tr><th>Col</th></tr><tr><td>A<br>B</td></tr></table>", options);
}

void main();

```
