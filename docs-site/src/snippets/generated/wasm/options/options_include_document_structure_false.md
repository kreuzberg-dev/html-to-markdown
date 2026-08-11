---
id: fixture_wasm_options_include_document_structure_false
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.includeDocumentStructure = false; return _u0; })();
  const result = convert("<article><h1>Heading</h1><p>Paragraph body.</p></article>", options);
}

void main();

```
