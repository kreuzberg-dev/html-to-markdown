---
id: fixture_wasm_options_preserve_tags_iframe
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { WasmConversionOptions, convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const options: WasmConversionOptions = (() => { const _u0 = WasmConversionOptions.default(); _u0.preserveTags = ["iframe"]; return _u0; })();
  const result = convert("<p>Before</p><iframe src='video.html' width='560'></iframe><p>After</p>", options);
}

void main();

```
