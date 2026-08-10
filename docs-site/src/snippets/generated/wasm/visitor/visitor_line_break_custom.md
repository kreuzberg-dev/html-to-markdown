```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitLineBreak(ctx: any): string | { Custom: string } {
        return { Custom: " | " };
    },

    }

  const result = convert("<p>First line<br>Second line<br>Third line</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
