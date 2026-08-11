---
id: fixture_wasm_visitor_horizontal_rule_custom
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
    visitHorizontalRule(ctx: any): string | { Custom: string } {
        return { Custom: "\n[DIVIDER]\n" };
    },

    }

  const result = convert("<h1>Section A</h1><p>Content A</p><hr><h1>Section B</h1><p>Content B</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
