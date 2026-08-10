```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.bullets = "*"; return _u0; })();
  const result = convert("<ul><li>Item A</li><li>Item B</li></ul>", options);
}

void main();

```
