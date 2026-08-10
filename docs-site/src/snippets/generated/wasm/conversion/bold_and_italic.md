```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><strong><em>both</em></strong></p>", undefined);
}

void main();

```
