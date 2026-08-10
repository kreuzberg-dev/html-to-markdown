```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitCustomElement(ctx: any, tagName: any, html: any): string | { Custom: string } {
        return "PreserveHtml";
    },

    }

  const result = convert("<div><custom-tag>Custom content</custom-tag></div>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
