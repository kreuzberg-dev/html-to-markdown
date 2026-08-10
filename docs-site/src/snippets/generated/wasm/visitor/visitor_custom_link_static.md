```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitLink(ctx: any, href: any, text: any, title: any): string | { Custom: string } {
        return { Custom: "[REDACTED LINK]" };
    },

    }

  const result = convert("<a href=\"https://example.com\">Click here</a>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
