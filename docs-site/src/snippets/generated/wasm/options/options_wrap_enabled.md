---
id: fixture_wasm_options_wrap_enabled
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.wrap = true; _u0.wrapWidth = 40; return _u0; })();
  const result = convert("<p>This is a long paragraph that should be wrapped at the specified column width when the wrap option is enabled.</p>", options);
}

void main();

```
