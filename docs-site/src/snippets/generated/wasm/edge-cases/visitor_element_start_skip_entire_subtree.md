```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitElementStart(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<div><h1>Title</h1><p>Content</p></div>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
