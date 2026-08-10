```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.excludeSelectors = [".cookie-banner"]; return _u0; })();
  const result = convert("<body><div class=\"cookie-banner\">Accept cookies</div><p>Main content</p></body>", options);
}

void main();

```
