---
id: fixture_wasm_options_preprocessing_minimal
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmPreprocessingOptions, WasmPreprocessingPreset, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.preprocessing = (() => { const _u1 = WasmPreprocessingOptions.default(); _u1.preset = WasmPreprocessingPreset.Minimal; return _u1; })(); return _u0; })();
  const result = convert("<nav>Navigation</nav><p>Content</p><footer>Footer</footer>", options);
}

void main();

```
