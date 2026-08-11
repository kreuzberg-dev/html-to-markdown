---
id: fixture_wasm_visitor_custom_blockquote
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
    visitBlockquote(ctx: any, content: any, depth: any): string | { Custom: string } {
        return { Custom: `QUOTE: "${content}"` };
    },

    }

  const result = convert("<blockquote><p>A wise quote.</p></blockquote>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
