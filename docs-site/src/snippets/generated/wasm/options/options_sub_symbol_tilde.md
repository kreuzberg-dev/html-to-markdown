```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.subSymbol = "~"; return _u0; })();
  const result = convert("<p>H<sub>2</sub>O</p>", options);
}

void main();

```
