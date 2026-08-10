```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.excludeSelectors = ["#ad-container"]; return _u0; })();
  const result = convert("<body><div id=\"ad-container\">Buy stuff</div><p>Article text</p></body>", options);
}

void main();

```
