---
id: fixture_wasm_options_code_language_python
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.codeLanguage = "python"; return _u0; })();
  const result = convert("<pre><code>def hello(): pass</code></pre>", options);
}

void main();

```
