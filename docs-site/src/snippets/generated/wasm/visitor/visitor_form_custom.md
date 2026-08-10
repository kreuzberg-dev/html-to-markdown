```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitForm(ctx: any, actionUrl: any, method: any): string | { Custom: string } {
        return { Custom: "[FORM PLACEHOLDER]" };
    },

    }

  const result = convert("<div><form action=\"/submit\" method=\"POST\"><label>Name: <input type=\"text\" name=\"name\"></label><button type=\"submit\">Submit</button></form></div>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
