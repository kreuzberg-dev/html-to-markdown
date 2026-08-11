---
id: fixture_wasm_options_newline_spaces
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmNewlineStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.newlineStyle = WasmNewlineStyle.Spaces; return _u0; })();
  const result = convert("<p>First<br>Second</p>", options);
}

void main();

```
