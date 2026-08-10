```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.extractMetadata = true; return _u0; })();
  const result = convert("<html><head><title>Scholarly Work</title><meta name=\"DC.title\" content=\"Principles of Knowledge Management\"><meta name=\"DC.creator\" content=\"Dr. Alice Johnson\"><meta name=\"DC.date\" content=\"2023-06-15\"><meta name=\"DC.subject\" content=\"Knowledge Management\"><meta name=\"DC.publisher\" content=\"Academic Press\"></head><body><p>This is a scholarly article.</p></body></html>", options);
}

void main();

```
