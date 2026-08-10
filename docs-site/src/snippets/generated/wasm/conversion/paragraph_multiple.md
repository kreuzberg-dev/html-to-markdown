```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>First paragraph.</p><p>Second paragraph.</p>", undefined);
}

void main();

```
