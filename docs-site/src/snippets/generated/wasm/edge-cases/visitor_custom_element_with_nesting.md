```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitCustomElement(ctx: any, tagName: any, html: any): string | { Custom: string } {
        return { Custom: "[CUSTOM WIDGET]" };
    },

    }

  const result = convert("<div><custom-widget data-value=\"123\"><p>Widget content here</p><span>With nested elements</span></custom-widget></div>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
