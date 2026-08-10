```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.brInTables = true; return _u0; })();
  const result = convert("<table><tr><th>Header</th></tr><tr><td>Line 1<br>Line 2</td></tr></table>", options);
}

void main();

```
