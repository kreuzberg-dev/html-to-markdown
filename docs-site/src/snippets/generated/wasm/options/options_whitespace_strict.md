```typescript title="WebAssembly"
import { WasmConversionOptions, WasmWhitespaceMode, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.whitespaceMode = WasmWhitespaceMode.Strict; return _u0; })();
  const result = convert("<p>Preserved   spacing.</p>", options);
}

void main();

```
