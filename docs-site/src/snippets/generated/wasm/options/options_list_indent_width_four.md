---
id: fixture_wasm_options_list_indent_width_four
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.listIndentWidth = 4; return _u0; })();
  const result = convert("<ul><li>Outer<ul><li>Inner</li></ul></li></ul>", options);
}

void main();

```
