```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.includeDocumentStructure = true; return _u0; })();
  const result = convert("<p>No tables here</p>", options);
}

void main();

```
