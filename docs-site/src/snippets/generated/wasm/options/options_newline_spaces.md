```typescript title="WebAssembly"
import { WasmConversionOptions, WasmNewlineStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.newlineStyle = WasmNewlineStyle.Spaces; return _u0; })();
  const result = convert("<p>First<br>Second</p>", options);
}

void main();

```
