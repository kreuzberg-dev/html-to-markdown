---
id: fixture_wasm_result_warning_kind_image_extraction_failed
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractImages = true; return _u0; })();
  const result = convert("<p>Text<img src=\"data:BADMIME\" alt=\"broken\">end</p>", options);
}

void main();

```
