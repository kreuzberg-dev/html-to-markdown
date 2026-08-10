```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.keepInlineImagesIn = ["p"]; return _u0; })();
  const result = convert("<p>Text <img src='icon.png' alt='icon'> more text</p>", options);
}

void main();

```
