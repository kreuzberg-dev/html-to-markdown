```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p>This paragraph is never closed", undefined);
}

void main();

```
