```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractMetadata = true; return _u0; })();
  const result = convert("<html><head><title>Docs</title></head><body><h1>Introduction</h1><h2>Getting Started</h2><h3>Installation</h3><h3>Configuration</h3><h2>Advanced Usage</h2><h3>Custom Options</h3></body></html>", options);
}

void main();

```
