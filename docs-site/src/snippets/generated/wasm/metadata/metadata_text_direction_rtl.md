```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractMetadata = true; return _u0; })();
  const result = convert("<html lang=\"ar\" dir=\"rtl\"><head><title>RTL Document</title></head><body><p>This is right-to-left text.</p></body></html>", options);
}

void main();

```
