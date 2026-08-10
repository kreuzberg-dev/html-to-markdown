```typescript title="WebAssembly"
import { WasmConversionOptions, WasmOutputFormat, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.outputFormat = WasmOutputFormat.Plain; return _u0; })();
  const result = convert("<h1>Title</h1><p>Some <strong>bold</strong> text.</p>", options);
}

void main();

```
