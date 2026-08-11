---
id: fixture_wasm_options_escape_underscores
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.escapeUnderscores = true; return _u0; })();
  const result = convert("<p>The variable_name is defined.</p>", options);
}

void main();

```
