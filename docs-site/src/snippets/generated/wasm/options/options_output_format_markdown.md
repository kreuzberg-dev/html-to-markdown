```typescript title="WebAssembly"
import { WasmConversionOptions, WasmHeadingStyle, WasmOutputFormat, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.headingStyle = WasmHeadingStyle.Atx; _u0.outputFormat = WasmOutputFormat.Markdown; return _u0; })();
  const result = convert("<h1>Title</h1><p>Some text.</p>", options);
}

void main();

```
