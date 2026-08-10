```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitImage(ctx: any, src: any, alt: any, title: any): string | { Custom: string } {
        return { Custom: `[Image: ${alt}]` };
    },

    }

  const result = convert("<img src=\"banner.png\" alt=\"Banner\">", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
