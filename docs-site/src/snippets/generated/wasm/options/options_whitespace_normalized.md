```typescript title="WebAssembly"
import { WasmConversionOptions, WasmWhitespaceMode, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.whitespaceMode = WasmWhitespaceMode.Normalized; return _u0; })();
  const result = convert("<p>Text   with    extra   spaces.</p>", options);
}

void main();

```
