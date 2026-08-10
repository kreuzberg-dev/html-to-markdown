```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.excludeSelectors = ["[role='complementary']"]; return _u0; })();
  const result = convert("<body><div role=\"complementary\">Sidebar</div><p>Primary text</p></body>", options);
}

void main();

```
