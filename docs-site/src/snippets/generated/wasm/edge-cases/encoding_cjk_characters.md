---
id: fixture_wasm_encoding_cjk_characters
language: typescript
target: wasm
level: typecheck
requires: []
side_effect: safe
---

```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>中文内容</p><p>日本語テキスト</p><p>한국어 텍스트</p>", undefined);
}

void main();

```
