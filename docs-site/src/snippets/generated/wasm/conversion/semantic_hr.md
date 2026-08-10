```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Above</p><hr><p>Below</p>", undefined);
}

void main();

```
