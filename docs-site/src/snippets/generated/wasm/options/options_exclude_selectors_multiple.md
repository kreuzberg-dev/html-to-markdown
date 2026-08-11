---
id: fixture_wasm_options_exclude_selectors_multiple
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.excludeSelectors = [".nav", "footer"]; return _u0; })();
  const result = convert("<body><nav class=\"nav\">Menu</nav><p>Content</p><footer>Footer</footer></body>", options);
}

void main();

```
