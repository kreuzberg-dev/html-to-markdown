```typescript title="WebAssembly"
import { WasmConversionOptions, WasmPreprocessingOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.preprocessing = (() => { const _u1 = WasmPreprocessingOptions.default(); _u1.enabled = false; return _u1; })(); return _u0; })();
  const result = convert("<nav>NavSection</nav><p>Paragraph</p>", options);
}

void main();

```
