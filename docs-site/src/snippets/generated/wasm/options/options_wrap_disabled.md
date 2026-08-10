```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.wrap = false; return _u0; })();
  const result = convert("<p>This is a long paragraph that should not be wrapped at all because wrapping is disabled.</p>", options);
}

void main();

```
