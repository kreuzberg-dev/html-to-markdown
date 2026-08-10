```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractMetadata = true; return _u0; })();
  const result = convert("<html><head><title>Page</title><meta name=\"keywords\" content=\"rust, markdown, html, converter\"></head><body><p>Content</p></body></html>", options);
}

void main();

```
