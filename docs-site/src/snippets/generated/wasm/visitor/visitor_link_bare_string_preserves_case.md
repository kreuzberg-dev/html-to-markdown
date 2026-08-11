---
id: fixture_wasm_visitor_link_bare_string_preserves_case
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
    visitLink(ctx: any, href: any, text: any, title: any): string | { Custom: string } {
        return `[${text}](https://new-cdn.com/file.pdf)`;
    },

    }

  const result = convert("<a href=\"https://old-cdn.com/file.pdf\">Download</a>", (() => { const _u = WasmConversionOptions.default(); _u.visitor = new WasmVisitorHandle(_testVisitor); return _u; })());
}

void main();

```
