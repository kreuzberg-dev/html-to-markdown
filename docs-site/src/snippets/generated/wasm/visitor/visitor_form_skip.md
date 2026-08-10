```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitForm(ctx: any, actionUrl: any, method: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Before form</p><form><input type=\"email\" name=\"email\"></form><p>After form</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
