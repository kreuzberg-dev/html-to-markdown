```typescript title="WebAssembly"
import { convert } from "@xberg-io/html-to-markdown-wasm";
function main() {
  const result = convert("<p><b><i>bold and italic</b></i></p>", undefined);
}

void main();

```
