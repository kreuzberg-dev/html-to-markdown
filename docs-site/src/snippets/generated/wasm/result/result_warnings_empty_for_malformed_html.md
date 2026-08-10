```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Unclosed paragraph<div>Mixed nesting</p></div>", undefined);
}

void main();

```
