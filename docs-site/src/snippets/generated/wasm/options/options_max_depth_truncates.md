---
id: fixture_wasm_options_max_depth_truncates
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.maxDepth = 3; return _u0; })();
  const result = convert("<div><p>Shallow</p><div><div><div><p>Too deep</p></div></div></div></div>", options);
}

void main();

```
