---
id: fixture_wasm_options_sup_symbol_caret
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.supSymbol = "^"; return _u0; })();
  const result = convert("<p>x<sup>2</sup></p>", options);
}

void main();

```
