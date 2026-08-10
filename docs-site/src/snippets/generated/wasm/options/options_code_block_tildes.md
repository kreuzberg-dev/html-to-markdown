```typescript title="WebAssembly"
import { WasmCodeBlockStyle, WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.codeBlockStyle = WasmCodeBlockStyle.Tildes; return _u0; })();
  const result = convert("<pre><code>let x = 1;</code></pre>", options);
}

void main();

```
