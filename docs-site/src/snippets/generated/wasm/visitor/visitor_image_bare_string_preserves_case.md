---
id: fixture_wasm_visitor_image_bare_string_preserves_case
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
    visitImage(ctx: any, src: any, alt: any, title: any): string | { Custom: string } {
        return `[image: ${alt} -> ${src}]`;
    },

    }

  const result = convert("<img src=\"PhotoOne.JPG\" alt=\"Sunset Over Bay\">", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
