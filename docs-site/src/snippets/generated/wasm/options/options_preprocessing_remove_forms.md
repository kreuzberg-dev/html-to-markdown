---
id: fixture_wasm_options_preprocessing_remove_forms
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmPreprocessingOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.preprocessing = (() => { const _u1 = WasmPreprocessingOptions.default(); _u1.removeForms = true; return _u1; })(); return _u0; })();
  const result = convert("<p>Before</p><form><input type='text'/><button>Submit</button></form><p>After</p>", options);
}

void main();

```
