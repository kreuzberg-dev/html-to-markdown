```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitMark(ctx: any, text: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Key insight: <mark>always validate input</mark> for security.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
