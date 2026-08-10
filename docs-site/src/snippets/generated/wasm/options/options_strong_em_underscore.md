```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.strongEmSymbol = "_"; return _u0; })();
  const result = convert("<p><strong>bold</strong> and <em>italic</em></p>", options);
}

void main();

```
