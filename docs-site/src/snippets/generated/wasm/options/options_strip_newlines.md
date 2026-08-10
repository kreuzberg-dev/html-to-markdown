```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.stripNewlines = true; return _u0; })();
  const result = convert("<p>First paragraph.</p><p>Second paragraph.</p>", options);
}

void main();

```
