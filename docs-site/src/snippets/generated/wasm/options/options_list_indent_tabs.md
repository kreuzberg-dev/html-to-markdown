```typescript title="WebAssembly"
import { WasmConversionOptions, WasmListIndentType, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.listIndentType = WasmListIndentType.Tabs; return _u0; })();
  const result = convert("<ul><li>Parent<ul><li>Child</li></ul></li></ul>", options);
}

void main();

```
