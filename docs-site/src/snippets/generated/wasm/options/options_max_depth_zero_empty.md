---
id: fixture_wasm_options_max_depth_zero_empty
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.maxDepth = 0; return _u0; })();
  const result = convert("<p>Hello</p>", options);
}

void main();

```
