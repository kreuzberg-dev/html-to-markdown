```typescript title="WebAssembly"
import { WasmCodeBlockStyle, WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.codeBlockStyle = WasmCodeBlockStyle.Indented; return _u0; })();
  const result = convert("<pre><code>print('hello')</code></pre>", options);
}

void main();

```
