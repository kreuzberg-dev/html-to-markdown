```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitSubscript(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `~${text}~` };
    },

    }

  const result = convert("<p>H<sub>2</sub>O is water.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
