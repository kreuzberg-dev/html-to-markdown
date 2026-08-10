```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><del>deleted text</del></p>", undefined);
}

void main();

```
