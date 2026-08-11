---
id: fixture_wasm_visitor_element_end_modification
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
    visitElementEnd(ctx: any, output: any): string | { Custom: string } {
        return { Custom: "MODIFIED OUTPUT" };
    },

    }

  const result = convert("<blockquote><p>Original quote</p></blockquote>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
