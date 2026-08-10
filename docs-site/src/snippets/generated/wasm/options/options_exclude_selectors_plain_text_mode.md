```typescript title="WebAssembly"
import { WasmConversionOptions, WasmOutputFormat, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.excludeSelectors = [".nav"]; _u0.outputFormat = WasmOutputFormat.Plain; return _u0; })();
  const result = convert("<body><div class=\"nav\">Navigation</div><p>Article body</p></body>", options);
}

void main();

```
