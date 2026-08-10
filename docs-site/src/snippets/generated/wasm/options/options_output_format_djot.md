```typescript title="WebAssembly"
import { WasmConversionOptions, WasmOutputFormat, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.outputFormat = WasmOutputFormat.Djot; return _u0; })();
  const result = convert("<p>Simple paragraph.</p>", options);
}

void main();

```
