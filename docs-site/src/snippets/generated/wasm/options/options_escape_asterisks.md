```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.escapeAsterisks = true; return _u0; })();
  const result = convert("<p>Use 2*3 = 6 in math.</p>", options);
}

void main();

```
