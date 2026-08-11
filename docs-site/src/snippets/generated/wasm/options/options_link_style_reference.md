---
id: fixture_wasm_options_link_style_reference
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmLinkStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.linkStyle = WasmLinkStyle.Reference; return _u0; })();
  const result = convert("<p><a href='https://example.com'>Example</a> and <a href='https://other.com'>Other</a></p>", options);
}

void main();

```
