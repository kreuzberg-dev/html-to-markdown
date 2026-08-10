```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.excludeSelectors = [".wrapper"]; return _u0; })();
  const result = convert("<body><div class=\"wrapper\"><p>Inner paragraph</p></div><p>Outer text</p></body>", options);
}

void main();

```
