```typescript title="WebAssembly"
import { WasmConversionOptions, WasmHighlightStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.highlightStyle = WasmHighlightStyle.None; return _u0; })();
  const result = convert("<p>Text with <mark>plain</mark> content.</p>", options);
}

void main();

```
