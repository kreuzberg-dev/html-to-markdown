---
id: fixture_wasm_visitor_underline_custom
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
    visitUnderline(ctx: any, text: any): string | { Custom: string } {
        return { Custom: `_${text}_` };
    },

    }

  const result = convert("<p>This is <u>very important</u> text.</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
