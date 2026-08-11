---
id: fixture_wasm_options_url_escape_style_percent_image
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmUrlEscapeStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.urlEscapeStyle = WasmUrlEscapeStyle.Percent; return _u0; })();
  const result = convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options);
}

void main();

```
