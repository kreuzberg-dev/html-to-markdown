```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractMetadata = true; return _u0; })();
  const result = convert("<html lang=\"en\" dir=\"ltr\"><head><title>LTR Document</title></head><body><p>This is left-to-right text.</p></body></html>", options);
}

void main();

```
