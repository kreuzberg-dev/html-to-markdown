```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitHeading(ctx: any, level: any, text: any, id: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<h1>Title</h1><p>Body text remains.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
