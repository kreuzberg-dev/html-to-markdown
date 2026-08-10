```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitDefinitionDescription(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `> ${text}` };
    },

    visitDefinitionTerm(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `### ${text}` };
    },

    }

  const result = convert("<dl><dt>Python</dt><dd>A high-level programming language</dd><dt>JavaScript</dt><dd>A scripting language for web browsers</dd></dl>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
