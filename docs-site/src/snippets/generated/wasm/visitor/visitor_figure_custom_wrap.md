```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitFigureEnd(ctx: any, output: any): string | { Custom: string } {
        return { Custom: `${output}
[/FIGURE]
` };
    },

    visitFigureStart(ctx: any): string | { Custom: string } {
        return { Custom: "\n[FIGURE]\n" };
    },

    }

  const result = convert("<section><h2>Gallery</h2><figure><img src=\"photo1.jpg\" alt=\"Photo\"><figcaption>Beautiful sunset</figcaption></figure></section>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
