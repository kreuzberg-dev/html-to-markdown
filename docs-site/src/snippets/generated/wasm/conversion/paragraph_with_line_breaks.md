```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>Line one.<br>Line two.<br>Line three.</p>", undefined);
}

void main();

```
