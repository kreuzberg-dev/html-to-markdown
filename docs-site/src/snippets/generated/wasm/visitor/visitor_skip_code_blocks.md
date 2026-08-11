---
id: fixture_wasm_visitor_skip_code_blocks
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
    visitCodeBlock(ctx: any, lang: any, code: any): string | { Custom: string } {
        return "Skip";
    },

    }

  const result = convert("<p>Intro text</p><pre><code>let x = 42;</code></pre><p>Outro text</p>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
