```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.skipImages = true; return _u0; })();
  const result = convert("<p>Before <img src='test.jpg' alt='photo'> After</p>", options);
}

void main();

```
