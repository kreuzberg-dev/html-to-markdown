---
id: fixture_wasm_options_encoding_utf8
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.encoding = "utf-8"; return _u0; })();
  const result = convert("<p>Café naïve résumé</p>", options);
}

void main();

```
