```typescript title="WebAssembly"
import { WasmConversionOptions, WasmUrlEscapeStyle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.urlEscapeStyle = WasmUrlEscapeStyle.Percent; return _u0; })();
  const result = convert("<img src=\"/img (1) <draft>.png\" alt=\"alt\">", options);
}

void main();

```
