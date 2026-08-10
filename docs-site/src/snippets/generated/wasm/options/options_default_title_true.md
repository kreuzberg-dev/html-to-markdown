```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.defaultTitle = true; return _u0; })();
  const result = convert("<p><a href='https://example.com'>Link</a></p>", options);
}

void main();

```
