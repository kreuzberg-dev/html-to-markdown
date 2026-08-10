```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.includeDocumentStructure = true; return _u0; })();
  const result = convert("<h1>Chapter One</h1><p>Chapter intro.</p><h2>Section One</h2><p>Section content.</p>", options);
}

void main();

```
