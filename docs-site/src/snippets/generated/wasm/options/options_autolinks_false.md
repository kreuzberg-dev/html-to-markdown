```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.autolinks = false; return _u0; })();
  const result = convert("<p><a href='https://example.com'>https://example.com</a></p>", options);
}

void main();

```
