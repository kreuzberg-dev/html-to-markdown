```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.escapeMisc = true; return _u0; })();
  const result = convert("<p>Use # and | and ~ in text.</p>", options);
}

void main();

```
