```typescript title="WebAssembly"
import { WasmConversionOptions, WasmHighlightStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.highlightStyle = WasmHighlightStyle.Bold; return _u0; })();
  const result = convert("<p>Text with <mark>highlighted</mark> text.</p>", options);
}

void main();

```
