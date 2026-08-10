```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitIframe(ctx: any, src: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<h3>Reviews</h3><iframe src=\"https://widget.example.com/reviews\"></iframe><p>See reviews from our partners.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
