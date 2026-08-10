```typescript title="WebAssembly"
import { WasmConversionOptions, WasmUrlEscapeStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.urlEscapeStyle = WasmUrlEscapeStyle.Angle; return _u0; })();
  const result = convert("<a href=\"/file (1).pdf\">file</a>", options);
}

void main();

```
