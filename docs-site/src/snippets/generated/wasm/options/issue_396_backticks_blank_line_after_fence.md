```typescript title="WebAssembly"
import { WasmCodeBlockStyle, WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.codeBlockStyle = WasmCodeBlockStyle.Backticks; return _u0; })();
  const result = convert("<p>Foo</p><pre><code>1\n2\n</code></pre><p>Bar</p>", options);
}

void main();

```
