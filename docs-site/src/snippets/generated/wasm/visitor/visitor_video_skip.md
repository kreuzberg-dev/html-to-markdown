```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitVideo(ctx: any, src: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<h2>Demo</h2><video src=\"demo.webm\"></video><p>See the demo above.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
