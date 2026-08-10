```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitLineBreak(ctx: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Address Line 1<br>Address Line 2<br>Address Line 3</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
