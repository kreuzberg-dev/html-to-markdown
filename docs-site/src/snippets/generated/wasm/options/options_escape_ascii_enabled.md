```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.escapeAscii = true; return _u0; })();
  const result = convert("<p>Text with # hash and [brackets] and * star</p>", options);
}

void main();

```
