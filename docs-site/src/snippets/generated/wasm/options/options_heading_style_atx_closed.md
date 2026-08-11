---
id: fixture_wasm_options_heading_style_atx_closed
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmHeadingStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.headingStyle = WasmHeadingStyle.AtxClosed; return _u0; })();
  const result = convert("<h1>Closed Heading</h1>", options);
}

void main();

```
