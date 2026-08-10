```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Before rule.</p><hr><p>After rule.</p>", undefined);
}

void main();

```
