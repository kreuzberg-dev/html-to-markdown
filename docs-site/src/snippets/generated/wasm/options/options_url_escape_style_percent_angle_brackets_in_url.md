---
id: fixture_wasm_options_url_escape_style_percent_angle_brackets_in_url
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
  const result = convert("<a href=\"/file (1) <draft>.pdf\">file</a>", options);
}

void main();

```
