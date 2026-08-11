---
id: fixture_wasm_visitor_custom_emphasis
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, WasmVisitorHandle, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const _testVisitor = {
    visitEmphasis(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `>>>${text}<<<` };
    },

    }

  const result = convert("<p>This is <em>important</em> text.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
