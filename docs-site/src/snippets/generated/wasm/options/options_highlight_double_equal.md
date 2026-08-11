---
id: fixture_wasm_options_highlight_double_equal
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmHighlightStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.highlightStyle = WasmHighlightStyle.DoubleEqual; return _u0; })();
  const result = convert("<p>Text with <mark>highlighted</mark> here.</p>", options);
}

void main();

```
