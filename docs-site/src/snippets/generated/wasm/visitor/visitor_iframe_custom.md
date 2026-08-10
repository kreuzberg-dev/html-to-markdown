```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitIframe(ctx: any, src: any): string | { Custom: string } {
        return { Custom: "[EMBEDDED: https://maps.example.com/embed]" };
    },

    }

  const result = convert("<p>Embedded map:</p><iframe src=\"https://maps.example.com/embed\" width=\"400\" height=\"300\"></iframe><p>End of map</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
