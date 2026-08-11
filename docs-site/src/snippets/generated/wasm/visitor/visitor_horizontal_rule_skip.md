---
id: fixture_wasm_visitor_horizontal_rule_skip
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
        return "Skip";
    },

    }

  const result = convert("<p>Part 1</p><hr><p>Part 2</p><hr><p>Part 3</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
